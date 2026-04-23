use std::{process::{exit}};
use axum::{ Extension, Router, http::{HeaderValue, Method, header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}} };
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions};
use tokio::net::TcpListener;
mod dtos;
mod models;
mod config;
mod errors;
mod db;
mod utils;
mod middleware;

use config::Config;
use db::DbClient;
use tower_http::cors::{CorsLayer};
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Clone)]
struct AppState{
    env: Config, 
    db_client: DbClient
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().with_max_level(LevelFilter::DEBUG).init();

    dotenv().ok();

    let config = Config::init();

    let pool = match PgPoolOptions::new().max_connections(10).connect(&config.database_url).await {
        Ok(ok) => {
            println!("Connected to database Succesfully");
            ok
        },
        Err(err) => {
            println!("Error Connection to DB: {}", err);
            exit(1)
        }
    };

    let cors = CorsLayer::new()
    .allow_credentials(true)
    .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
    .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap());

    let db_client = DbClient::new(pool);

    let app_state = AppState{
        env: config.clone(),
        db_client
    };

    let app = Router::new().layer(Extension(app_state)).layer(cors.clone());

    // let app = Router::new().route("/", get(root));

    let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await.unwrap();
    
    println!("Server is running on port: {}", config.port);

    axum::serve(listener, app).await.unwrap()

}
