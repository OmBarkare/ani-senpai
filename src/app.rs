use crate::anilist::Anime;
use crate::gemini::Recommendation;

#[derive(Clone, Copy, PartialEq)]
pub enum Focus {
    Gemini,
    AniList,
}

#[derive(Clone)]
pub struct App {
    pub focus: Focus,
    pub gemini_index: usize,
    pub anilist_index: usize,
    pub page: i32,

    pub gemini_recs: Vec<Recommendation>,
    pub anilist_items: Vec<Anime>,

    pub genres: Vec<String>,
    pub tags: Vec<String>,
    pub has_next_page: bool,

    pub status_message: String,
    pub should_quit: bool,
}

impl App {
    pub fn new(gemini_recs: Vec<Recommendation>) -> Self {
        Self {
            focus: Focus::Gemini,
            gemini_index: 0,
            anilist_index: 0,
            page: 1,
            gemini_recs,
            anilist_items: Vec::new(),
            genres: Vec::new(),
            tags: Vec::new(),
            has_next_page: false,
            status_message: String::new(),
            should_quit: false,
        }
    }
}