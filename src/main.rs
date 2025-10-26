mod config;
mod db;
mod error;
mod handlers;
mod models;
mod services;
mod utils;

use actix_web::{middleware::Logger, web, App, HttpServer};
use std::fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let config = config::Config::from_env();
    fs::create_dir_all("cache").expect("Failed to create cache directory");

    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    println!("Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations completed");

    let server_host = config.server_host.clone();
    let server_port = config.server_port;

    log::info!(
        "Starting server at {}:{}",
        server_host,
        server_port
    );

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(config.clone()))
            .configure(handlers::configure_routes)
    })
    .bind((server_host.as_str(), server_port))?
    .run()
    .await
}