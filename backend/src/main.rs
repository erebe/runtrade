#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod api;

use crate::db::get_postgres_connection_pool;
use std::env;
use actix_web::{HttpServer, App, middleware, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    // Logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let db_pool = get_postgres_connection_pool(&database_url)
        .expect("Cannot create postgres connection pool");

    let port = std::env::var("PORT").unwrap_or("8081".to_string());
    HttpServer::new(move || App::new()
        .wrap(middleware::DefaultHeaders::new().header("Access-Control-Allow-Origin", "http://localhost:8080"))
        .wrap(middleware::Logger::default())
        .data(db_pool.clone())
        .data(web::JsonConfig::default().limit(4096))
        .service(api::routes::get_user_by_id)
        .service(api::routes::get_user_by_name)
        .service(api::routes::find_events)
    )
        .bind(format!("0.0.0.0:{}", &port))?
        .run()
        .await
}

