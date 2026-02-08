use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState},
};

use crate::app::{App, Focus};

pub fn draw(frame: &mut Frame, app: &App) {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(67),
            Constraint::Length(3),
        ])
        .split(frame.size());

    // Gemini list
    let gemini_items: Vec<ListItem> = app
        .gemini_recs
        .iter()
        .map(|r| ListItem::new(format!("{}\n  → {}", r.title, r.reason)))
        .collect();

    let mut gemini_state = ListState::default();
    gemini_state.select(Some(app.gemini_index));

    let gemini_border_style = if app.focus == Focus::Gemini {
        Style::default().fg(Color::Magenta)
    } else {
        Style::default()
    };

    let gemini = List::new(gemini_items)
        .block(
            Block::default()
                .title("🤖 Gemini Picks")
                .borders(Borders::ALL)
                .border_style(gemini_border_style)
        )
        .highlight_style(Style::default().bg(Color::Magenta).fg(Color::Black));

    // AniList list
    let anilist_items: Vec<ListItem> = app
        .anilist_items
        .iter()
        .map(|a| ListItem::new(a.title.romaji.clone()))
        .collect();

    let mut anilist_state = ListState::default();
    anilist_state.select(Some(app.anilist_index));

    let anilist_border_style = if app.focus == Focus::AniList {
        Style::default().fg(Color::Blue)
    } else {
        Style::default()
    };

    let anilist = List::new(anilist_items)
        .block(
            Block::default()
                .title(format!("📺 AniList Results ({})", app.anilist_items.len()))
                .borders(Borders::ALL)
                .border_style(anilist_border_style)
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(gemini, layout[0], &mut gemini_state);
    frame.render_stateful_widget(anilist, layout[1], &mut anilist_state);

    // Footer
    let footer_text = format!(
        "[↑↓] move  [Tab] switch  [Enter] play  [q] quit  |  Focus: {}",
        match app.focus {
            Focus::Gemini => "Gemini",
            Focus::AniList => "AniList",
        }
    );

    let footer = Block::default()
        .borders(Borders::ALL)
        .title(footer_text)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(footer, layout[2]);
}