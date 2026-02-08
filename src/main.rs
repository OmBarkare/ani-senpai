mod anilist;
mod gemini;

use anilist::fetch_anime_page;
use gemini::get_gemini_recommendations;
use reqwest::Error;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let genres = vec![
    //     "Psychological".to_string(),
    //     "Sci-Fi".to_string(),
    // ];

    // let tags = vec![
    //     "Cyberpunk".to_string(),
    //     "Existentialism".to_string(),
    // ];

    print!("Describe what you want to watch: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();

    // Fallback defaults (important)
    let mut genres = vec!["Psychological".to_string(), "Sci-Fi".to_string()];
    let mut tags: Vec<String> = Vec::new();

    // Try Gemini
    match get_gemini_recommendations(query.trim()).await {
        Ok(gemini) => {
            println!("\nGemini recommendations:");
            for (i, rec) in gemini.recommendations.iter().enumerate() {
                println!("{}. {} — {}", i + 1, rec.title, rec.reason);
            }

            // Use Gemini genres
            if gemini.genres.len() == 3 {
                genres = gemini.genres;
            }

            // Use Gemini tag only if present
            if let Some(tag) = gemini.tag {
                tags = vec![tag];
            }
        }
        Err(e) => {
            println!("Gemini failed, using fallback filters: {}", e);
        }
    }

    let mut page = 1;

    

    loop {
        let result = fetch_anime_page(page, 5, genres.clone(), tags.clone()).await?;
        
        println!("\nPage {}", page);
        for (i, anime) in result.media.iter().enumerate() {
            println!("{}. {}", i + 1, anime.title.romaji);
        }

        if !result.pageInfo.hasNextPage {
            println!("\nNo more pages.");
            break;
        }

        print!("\n[n] next page | [q] quit > ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim() {
            "n" => {
                if result.pageInfo.hasNextPage {
                    page += 1;
                } else {
                    println!("No next page available");
                }
            },
            "p" => if page > 1 {
                page -= 1;
            } else {
                 println!("Cannot go to previous page");
            }
            "q" => break,
            _ => println!("Invalid input"),
                other => {
        // Try numeric selection
        if let Ok(index) = other.parse::<usize>() {
            if index >= 1 && index <= result.media.len() {
                let anime = &result.media[index - 1];
                launch_ani_cli(&anime.title.romaji)?;
                break;
            } else {
                println!("Invalid selection");
            }
        } else {
            println!("Invalid input");
        }
    }
        }
    }

    Ok(())
}

fn launch_ani_cli(title: &String) -> Result<(), Error>{
    todo!()
}