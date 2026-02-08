use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct GeminiResponse {
    pub genres: Vec<String>,
    pub tag: Option<String>,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Debug, Deserialize)]
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
You are an anime recommendation assistant.

Task:
- Choose exactly 3 AniList-valid genres
- Choose at most 1 AniList-valid tag
- Recommend exactly 3 anime
- Provide a short reason (max 50 words) for each

Rules:
- ReN
- Do NOT use markdturn ONLY valid JSOown
- Do NOT add explanations
- Use official AniList anime titles
- If unsure, choose common genres and no tag

JSON format:
{{
  "genres": [string, string, string],
  "tag": string | null,
  "recommendations": [
    {{ "title": string, "reason": string }},
    {{ "title": string, "reason": string }},
    {{ "title": string, "reason": string }}
  ]
}}

User input:
"{input}"
"#,
        input = user_input
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
