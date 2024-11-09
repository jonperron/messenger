mod config;
mod templates;
mod providers;

use axum:: Router;
use crate::config::Config;
use crate::templates::tera_engine::TemplateEngine;

#[tokio::main]
async fn main() {
    let config = Config::load_from_file("config.yaml").expect("Failed to load config");

    let app = Router::new();

    let addr: std::net::SocketAddr = format!("0.0.0.0:{}", config.service.port).parse().unwrap();
    if config.service.environment == "development" {
        println!("Listening on http://{}", addr);
    }

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}