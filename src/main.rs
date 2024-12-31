// Declare modules part
mod config;
mod api;
mod models;
mod templates_engines;
mod providers;
mod tools;

use std::sync::Arc;
use std::net::SocketAddr;

use axum::{
    Router,
    routing::get,
};
use crate::api::send_router;
use crate::config::Config;
use crate::providers::MailgunProvider;
use crate::templates_engines::tera_engine::TemplateEngine;
use crate::tools::health_handler;


#[tokio::main]
async fn main() {
    // Load the configuration
    let config = Arc::new(Config::load_from_file("config.yaml").expect("Failed to load config"));

    // Init dependencies
    let template_engine = Arc::new(TemplateEngine::new("templates").expect("Failed to load template engine"));
    let mailgun_provider = Arc::new(MailgunProvider::new(config.mailgun.clone()));

    let app = send_router(template_engine, mailgun_provider, config.clone())
        .merge(Router::new().route("/health", get(health_handler)));

    let addr: SocketAddr = format!("0.0.0.0:{}", config.service.port).parse().unwrap();
    if config.service.environment == "development" {
        println!("Listening on http://{}", addr);
    }

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}