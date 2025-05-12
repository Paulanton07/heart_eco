use actix_cors::Cors;
use actix_web::{middleware, web, App, HttpServer};
use actix_web::http::header;
use env_logger::Env;
use std::io;

mod config;
mod models;
mod handlers;
mod services;
mod utils;
mod ai;

use config::Config;

#[actix_web::main]
async fn main() -> io::Result<()> {
    // Initialize environment variables and config
    let config = Config::init();
    
    // Set up logging
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    log::info!("Starting server at http://localhost:{}", config.server_port);

    // Start HTTP server
    HttpServer::new(move || {
        // Configure CORS
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                config.allowed_origins.iter().any(|allowed| allowed == origin.as_bytes())
            })
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);
        
        App::new()
            // Enable logger middleware
            .wrap(middleware::Logger::default())
            // Enable CORS
            .wrap(cors)
            // App data and state
            .app_data(web::Data::new(config.clone()))
            // API routes will be added here
            .route("/", web::get().to(|| async { "Wood Planks E-commerce API" }))
            .route("/health", web::get().to(|| async { "OK" }))
    })
    .bind(("0.0.0.0", config.server_port))?
    .run()
    .await
}
