use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Movie {
    pub id: i32,
    pub title: Option<String>,
    pub name: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub vote_average: Option<f64>,
    pub release_date: Option<String>,
    pub media_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TmdbResponse {
    pub page: i32,
    pub results: Vec<Movie>,
    pub total_pages: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Video {
    pub id: String,
    pub key: String,
    pub site: String,
    pub r#type: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VideoResponse {
    pub id: String,
    pub results: Vec<Movie>,
}

#[derive(Deserialize)]
pub struct PageQuery {
    pub page: Option<i32>,
}

#[derive(Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub page: Option<i32>,
}

