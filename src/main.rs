use axum::{routing::get, Router};
use dotenv::dotenv;
use std::env;
use tokio::net::TcpListener;
use tower_http::{services::ServeDir, cors::CorsLayer};

mod handlers;
mod models;
mod state;

use state::AppState;
use handlers::{root, get_movie_videos, get_trending_movies, search_content};

#[tokio::main]
async fn main() {
    dotenv().ok();
    let tmdb_api_key = env::var("TMDB_API_KEY").unwrap();
    let app_state = AppState { tmdb_api_key, client: reqwest::Client::new() };
    let cors = CorsLayer::new().allow_origin(tower_http::cors::Any);
    let app = Router::new()
        .route("/", get(root))
        .route("/api/search", get(search_content))
        .route("/api/trending", get(get_trending_movies))
        .route("/api/movie/{id}/videos", get(get_movie_videos))
        .nest_service("/stream", ServeDir::new("assets"))
        .layer(cors)
        .with_state(app_state);
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("Serving listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}