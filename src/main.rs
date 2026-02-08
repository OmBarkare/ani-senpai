mod anilist;
mod gemini;
mod app;
mod ui;

use std::io::Write;
use anilist::fetch_anime_page;
use gemini::get_gemini_recommendations;
use app::{App, Focus};

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{enable_raw_mode, disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};

use ratatui::{Terminal, backend::CrosstermBackend};
use std::{io::{self, stdout}, process::Command, time::Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ---- Fetch Gemini ----
    print!("Describe what you want to watch: ");
    io::stdout().flush().unwrap();

    let mut query = String::new();
    io::stdin().read_line(&mut query).unwrap();

    let gemini_response = match get_gemini_recommendations(query.trim()).await {
        Ok(resp) => resp,
        Err(e) => {
            eprintln!("Gemini error: {}", e);
            return Ok(());
        }
    };

    let mut app = App::new(gemini_response.recommendations.clone());
    app.genres = gemini_response.genres;
    app.tags = if let Some(tag) = gemini_response.tag {
        vec![tag]
    } else {
        vec![]
    };

    // ---- Terminal setup ----
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    // ---- Initial AniList fetch using Gemini's genres/tags ----
    app.anilist_items = match fetch_anime_page(
        app.page,
        10,
        app.genres.clone(),
        app.tags.clone(),
    )
    .await {
        Ok(page) => {
            app.has_next_page = page.pageInfo.hasNextPage;
            page.media
        },
        Err(e) => {
            // Clean up terminal before showing error
            disable_raw_mode()?;
            execute!(stdout(), LeaveAlternateScreen)?;
            eprintln!("AniList error: {}", e);
            return Ok(());
        }
    };

    // ---- Main loop ----
    let result = run_app(&mut terminal, &mut app).await;

    // ---- Restore terminal ----
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;

    result
}

async fn run_app(
    terminal: &mut Terminal<CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
) -> Result<(), Box<dyn std::error::Error>> {
    loop {
        terminal.draw(|f| ui::draw(f, &app))?;

        // Clear any status message after displaying
        if !app.status_message.is_empty() {
            app.status_message.clear();
        }

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => {
                        app.should_quit = true;
                        break;
                    }

                    KeyCode::Tab => {
                        app.focus = match app.focus {
                            Focus::Gemini => Focus::AniList,
                            Focus::AniList => Focus::Gemini,
                        };
                    }

                    KeyCode::Up => match app.focus {
                        Focus::Gemini if app.gemini_index > 0 => app.gemini_index -= 1,
                        Focus::AniList if app.anilist_index > 0 => app.anilist_index -= 1,
                        _ => {}
                    },

                    KeyCode::Down => match app.focus {
                        Focus::Gemini if app.gemini_index + 1 < app.gemini_recs.len() => {
                            app.gemini_index += 1;
                        }
                        Focus::AniList if app.anilist_index + 1 < app.anilist_items.len() => {
                            app.anilist_index += 1;
                        }
                        // Auto-fetch next page when scrolling past the bottom
                        Focus::AniList if app.anilist_index + 1 >= app.anilist_items.len() && app.has_next_page => {
                            app.page += 1;
                            match fetch_anime_page(
                                app.page,
                                10,
                                app.genres.clone(),
                                app.tags.clone(),
                            )
                            .await {
                                Ok(page) => {
                                    app.anilist_items = page.media;
                                    app.has_next_page = page.pageInfo.hasNextPage;
                                    app.anilist_index = 0;
                                    app.status_message = format!("Loaded page {}", app.page);
                                },
                                Err(_) => {
                                    app.page -= 1;
                                    app.status_message = "Failed to load next page".to_string();
                                }
                            }
                        }
                        Focus::AniList if app.anilist_index + 1 >= app.anilist_items.len() && !app.has_next_page => {
                            app.status_message = "No more pages available".to_string();
                        }
                        _ => {}
                    },

                    KeyCode::Right if app.focus == Focus::AniList && app.has_next_page => {
                        // Fetch next page
                        app.page += 1;
                        match fetch_anime_page(
                            app.page,
                            10,
                            app.genres.clone(),
                            app.tags.clone(),
                        )
                        .await {
                            Ok(page) => {
                                app.anilist_items = page.media;
                                app.has_next_page = page.pageInfo.hasNextPage;
                                app.anilist_index = 0;
                                app.status_message = format!("Page {}", app.page);
                            },
                            Err(_) => {
                                app.page -= 1;
                                app.status_message = "Failed to load page".to_string();
                            }
                        }
                    }
                    
                    KeyCode::Right if app.focus == Focus::AniList && !app.has_next_page => {
                        app.status_message = "No more pages available".to_string();
                    }

                    KeyCode::Left if app.focus == Focus::AniList && app.page > 1 => {
                        // Fetch previous page
                        app.page -= 1;
                        match fetch_anime_page(
                            app.page,
                            10,
                            app.genres.clone(),
                            app.tags.clone(),
                        )
                        .await {
                            Ok(page) => {
                                app.anilist_items = page.media;
                                app.has_next_page = page.pageInfo.hasNextPage;
                                app.anilist_index = 0;
                                app.status_message = format!("Page {}", app.page);
                            },
                            Err(_) => {
                                app.page += 1;
                                app.status_message = "Failed to load page".to_string();
                            }
                        }
                    }

                    KeyCode::Enter => {
                        let title = match app.focus {
                            Focus::Gemini =>
                                app.gemini_recs[app.gemini_index].title.clone(),
                            Focus::AniList =>
                                app.anilist_items[app.anilist_index].title.romaji.clone(),
                        };

                        // Store current state
                        let saved_state = app.clone();

                        // Clean up terminal before launching ani-cli
                        disable_raw_mode()?;
                        execute!(stdout(), LeaveAlternateScreen)?;
                        
                        launch_ani_cli(&title)?;
                        
                        // Restore terminal after ani-cli exits
                        enable_raw_mode()?;
                        execute!(stdout(), EnterAlternateScreen)?;
                        
                        // Restore the app state
                        *app = saved_state;
                        app.status_message = format!("Returned from watching: {}", title);
                        
                        // Force a full redraw
                        terminal.clear()?;
                    }

                    _ => {}
                }
            }
        }
    }

    Ok(())
}

fn launch_ani_cli(title: &str) -> Result<(), Box<dyn std::error::Error>> {
    Command::new("ani-cli").arg(title).status()?;
    Ok(())
}