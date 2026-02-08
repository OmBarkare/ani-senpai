
mod anilist;

use anilist::fetch_anime_page;
use std::io::{self, Write};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let genres = vec![
        "Psychological".to_string(),
        "Sci-Fi".to_string(),
    ];

    let tags = vec![
        "Cyberpunk".to_string(),
        "Existentialism".to_string(),
    ];

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
            "n" => page += 1,
            "q" => break,
            _ => println!("Invalid input"),
        }
    }

    Ok(())
}
