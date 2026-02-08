use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct AniListQuery {
    pub query: String,
    pub variables: Variables,
}

#[derive(Debug, Serialize)]
pub struct Variables {
    pub page: i32,
    pub perPage: i32,
    pub genres: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct AniListResponse {
    pub data: PageData,
}

#[derive(Debug, Deserialize)]
pub struct PageData {
    #[serde(rename = "Page")]
    pub page: Page,
}

#[derive(Debug, Deserialize)]
pub struct Page {
    pub pageInfo: PageInfo,
    pub media: Vec<Anime>,
}

#[derive(Debug, Deserialize)]
pub struct PageInfo {
    pub hasNextPage: bool,
}

#[derive(Debug, Deserialize)]
pub struct Anime {
    pub id: i32,
    pub title: Title,
}

#[derive(Debug, Deserialize)]
pub struct Title {
    pub romaji: String,
}

const ANILIST_ENDPOINT: &str = "https://graphql.anilist.co";

const QUERY: &str = r#"
query ($page: Int, $perPage: Int, $genres: [String], $tags: [String]) {
  Page(page: $page, perPage: $perPage) {
    pageInfo {
      hasNextPage
    }
    media(
      type: ANIME
      genre_in: $genres
      tag_in: $tags
      sort: SCORE_DESC
    ) {
      id
      title {
        romaji
      }
      averageScore
    }
  }
}
"#;


pub async fn fetch_anime_page(
    page: i32,
    perPage: i32,
    genres: Vec<String>,
    tags: Vec<String>,
) -> Result<Page, reqwest::Error> {
    let client = Client::new();

    let body = AniListQuery {
        query: QUERY.to_string(),
        variables: Variables {
            page,
            perPage,
            genres,
            tags,
        },
    };

    let res = client
        .post(ANILIST_ENDPOINT)
        .json(&body)
        .send()
        .await?
        .json::<AniListResponse>()
        .await?;

    Ok(res.data.page)
}
