use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub genres: Vec<String>,
    pub tag: Option<String>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Recommendation {
    pub title: String,
    pub reason: String,
}

use reqwest::Client;
use serde_json::json;
use std::env;

const GEMINI_ENDPOINT: &str = "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent";

pub async fn get_gemini_recommendations(
    user_input: &str,
) -> Result<GeminiResponse, Box<dyn std::error::Error>> {
    let api_key = env::var("GEMINI_API_KEY").expect("GEMINI_API_KEY not set");

    let prompt = format!(
        r#"
You are an anime recommendation assistant that provides personalized suggestions based on user preferences.

TASK:
1. Analyze the user's input carefully
2. If the user mentions a SPECIFIC anime title they want to watch, include it as the FIRST recommendation
3. Choose exactly 3 valid AniList genres that match the user's request
4. Choose at most 1 valid AniList tag (optional)
5. Provide exactly 5 anime recommendations total
6. Give a concise reason (max 40 words) for each recommendation

CRITICAL RULES:
- Return ONLY valid JSON with no markdown, no code blocks, no explanations
- Use EXACT official AniList anime titles (correct spelling and capitalization)
- If user mentions a specific anime, it MUST be recommendation #1
- Genres must be valid AniList genres: Action, Adventure, Comedy, Drama, Ecchi, Fantasy, Horror, Mahou Shoujo, Mecha, Music, Mystery, Psychological, Romance, Sci-Fi, Slice of Life, Sports, Supernatural, Thriller
- Tags are optional but must be valid AniList tags if used
- Recommendations should be diverse but thematically related
- Prioritize highly-rated and popular anime

VALID JSON FORMAT (no other text):
{{
  "genres": ["Genre1", "Genre2", "Genre3"],
  "tag": "TagName" or null,
  "recommendations": [
    {{"title": "Exact Anime Title", "reason": "Brief explanation"}},
    {{"title": "Exact Anime Title", "reason": "Brief explanation"}},
    {{"title": "Exact Anime Title", "reason": "Brief explanation"}},
    {{"title": "Exact Anime Title", "reason": "Brief explanation"}},
    {{"title": "Exact Anime Title", "reason": "Brief explanation"}}
  ]
}}

USER INPUT:
"{}"

RESPOND WITH ONLY THE JSON OBJECT:"#,
        user_input
    );

    let body = json!({
        "contents": [{
            "parts": [{ "text": prompt }]
        }]
    });

    let client = Client::new();
    let raw = client
        .post(format!("{}?key={}", GEMINI_ENDPOINT, api_key))
        .json(&body)
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    println!("{:?}", raw);

    let text = raw["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .ok_or("Invalid Gemini response")?;

    let json_str = extract_json(text)?;
    let parsed: GeminiResponse = serde_json::from_str(json_str)?;

    Ok(parsed)
}

fn extract_json(text: &str) -> Result<&str, &'static str> {
    let start = text.find('{').ok_or("No JSON found")?;
    let end = text.rfind('}').ok_or("No JSON found")?;
    Ok(&text[start..=end])
}
