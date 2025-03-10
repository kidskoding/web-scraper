extern crate tokio;
use std::fs;

use axum::{response::Html, routing::get, Router};
use reqwest::StatusCode;

pub async fn serve() -> Result<(), Box<dyn std::error::Error>> {
    let app = Router::new().route("/", get(index));
    let url = "localhost:3000";
    let listener = tokio::net::TcpListener::bind(url).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

async fn index() -> Result<Html<String>, (StatusCode, String)> {
    match fs::read_to_string("static/index.html") {
        Ok(content) => Ok(Html(content)),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Failed to read HTML file: {}", err),
        )),
    }
}
