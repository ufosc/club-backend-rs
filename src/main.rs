mod auth;

#[macro_use]
extern crate diesel;

use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use env_logger::Env;
use log::{info};

mod models;
mod schema;
mod errors;
mod auth_handler;
mod user_handler;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::from_env(Env::default().default_filter_or("debug")).init();

    info!("Creating database connection pool");
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create r2d2 databse pool.");
    info!("Database connection pool created successfully.");
    
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("8088".to_string());

    info!("Starting HttpServer");
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(Logger::default())
            .service(
                web::scope("/api")
                    .service(web::scope("/users").configure(user_handler::config))
                    .service(web::scope("/auth").configure(auth_handler::config)),
            )
            .route("/ping", web::get().to(|| HttpResponse::Ok().body("pong")))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
