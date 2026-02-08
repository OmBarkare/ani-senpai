mod anilist;
mod gemini;

use anilist::fetch_anime_page;
use gemini::get_gemini_recommendations;
use crate::gemini::{GeminiResponse, Recommendation};
use std::io::{self, Write};
use std::process::Command;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Prompt
    print!("Describe what you want to watch: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();
    let query = query.trim();

    // Fallback defaults
    let mut genres = vec!["Psychological".to_string(), "Sci-Fi".to_string()];
    let mut tags: Vec<String> = Vec::new();

    // Try Gemini (optional)
    let gemini_response: Option<GeminiResponse> = match get_gemini_recommendations(query).await {
        Ok(resp) => {
            println!("\nGemini recommendations:");
            for (i, rec) in resp.recommendations.iter().enumerate() {
                println!("g{}. {} — {}", i + 1, rec.title, rec.reason);
            }

            // adopt genres if valid
            if resp.genres.len() == 3 {
                genres = resp.genres.clone();
            }

            // adopt single tag if present
            if let Some(tag) = &resp.tag {
                tags = vec![tag.clone()];
            }

            Some(resp)
        }
        Err(e) => {
            println!("Gemini failed, using fallback filters: {}", e);
            None
        }
    };

    // Prepare a vector of Gemini recommendations (possibly empty)
    let gemini_recs = gemini_response
        .as_ref()
        .map(|r| r.recommendations.clone())
        .unwrap_or_else(Vec::new);

    // start the interactive loop; it will only return when the user quits (presses 'q')
    call_loop(1, genres, tags, gemini_recs).await?;

    println!("Goodbye.");
    Ok(())
}

/// Runs the interactive pagination + selection loop.
/// When a title is selected, launch_ani_cli(...) is called and control returns to the loop afterwards.
async fn call_loop(
    mut page: i32,
    genres: Vec<String>,
    tags: Vec<String>,
    gemini_recs: Vec<Recommendation>,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        let result = fetch_anime_page(page, 5, genres.clone(), tags.clone()).await?;

        if result.media.is_empty() {
            println!("\nNo results found on this page.");
            if !result.pageInfo.hasNextPage {
                // nothing more to do
                break;
            }
        } else {
            println!("\nPage {}", page);
            for (i, anime) in result.media.iter().enumerate() {
                println!("{}. {}", i + 1, anime.title.romaji);
            }
        }

        println!(
            "\n[n] next | [p] prev | [g1-g{}] Gemini | [1-{}] select | [q] quit",
            gemini_recs.len(),
            result.media.len()
        );

        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "n" => {
                if result.pageInfo.hasNextPage {
                    page += 1;
                } else {
                    println!("No next page available");
                }
            }
            "p" => {
                if page > 1 {
                    page -= 1;
                } else {
                    println!("Cannot go to previous page");
                }
            }
            "q" => break,
            other => {
                // Gemini selection: gN
                if other.starts_with('g') {
                    let idx_str = &other[1..];
                    match idx_str.parse::<usize>() {
                        Ok(idx) if idx >= 1 && idx <= gemini_recs.len() => {
                            let title = gemini_recs[idx - 1].title.clone();
                            // launch ani-cli; when it exits, we return here and continue loop
                            if let Err(e) = launch_ani_cli(&title) {
                                eprintln!("Failed to run ani-cli: {}", e);
                            }
                            continue; // continue showing the list after ani-cli exits
                        }
                        _ => {
                            println!("Invalid Gemini selection (use g1..gN)");
                            continue;
                        }
                    }
                }

                // AniList numeric selection
                if let Ok(idx) = other.parse::<usize>() {
                    if idx >= 1 && idx <= result.media.len() {
                        let title = result.media[idx - 1].title.romaji.clone();
                        if let Err(e) = launch_ani_cli(&title) {
                            eprintln!("Failed to run ani-cli: {}", e);
                        }
                        continue; // return to list after ani-cli exits
                    } else {
                        println!("Invalid selection (out of range)");
                        continue;
                    }
                }

                println!("Invalid input");
            }
        }
    }

    Ok(())
}

/// Launches ani-cli synchronously and returns after it exits.
/// Uses Command::status so the user interacts with ani-cli in the same terminal.
fn launch_ani_cli(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("\nLaunching ani-cli for: {}\n", title);

    let status = Command::new("ani-cli").arg(title).status()?;

    if !status.success() {
        eprintln!("ani-cli exited with status: {}", status);
    }

    Ok(())
}
