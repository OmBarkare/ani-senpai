use ratatui::{
    prelude::*,
    widgets::{Block, Borders, List, ListItem, ListState, Paragraph},
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

    let page_indicator = if app.has_next_page {
        format!(" (Page {}) →", app.page)
    } else {
        format!(" (Page {} - Last)", app.page)
    };

    let anilist = List::new(anilist_items)
        .block(
            Block::default()
                .title(format!("📺 AniList Results{}", page_indicator))
                .borders(Borders::ALL)
                .border_style(anilist_border_style)
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

    frame.render_stateful_widget(gemini, layout[0], &mut gemini_state);
    frame.render_stateful_widget(anilist, layout[1], &mut anilist_state);

    // Footer with status message
    let footer_text = if !app.status_message.is_empty() {
        format!("⚠️  {} | [q/Esc] quit", app.status_message)
    } else {
        format!(
            "[↑↓] move  [←→] page  [Tab] switch  [Enter] play  [q/Esc] quit  |  Focus: {}",
            match app.focus {
                Focus::Gemini => "Gemini",
                Focus::AniList => "AniList",
            }
        )
    };

    let footer_style = if !app.status_message.is_empty() {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let footer = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(footer_style)
        );

    frame.render_widget(footer, layout[2]);
}