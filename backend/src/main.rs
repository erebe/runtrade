#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate lazy_static;

mod db;
mod api;

use crate::db::get_postgres_connection_pool;
use std::env;
use actix_web::{HttpServer, App, middleware, web, http};


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
    HttpServer::new(move || {
        let mut headers = middleware::DefaultHeaders::new();
        #[cfg(debug_assertions)] // If in not release allow localhost to ease developpement
        (headers = headers.header("Access-Control-Allow-Origin", "http://localhost:8080"));

        let app = App::new()
            .wrap(headers)
            .wrap(middleware::Logger::default())
            .data(db_pool.clone())
            .data(web::JsonConfig::default().limit(4096))
            .service(api::routes::get_user_by_id)
            .service(api::routes::get_user_by_name)
            .service(api::routes::update_user_contact)
            .service(api::routes::put_user_logged)
            .service(api::routes::find_events)
            .service(api::routes::get_events_types)
            .service(api::routes::get_inscriptions_by_event_id)
            .service(api::routes::add_event)
            .service(api::routes::add_inscription)
            .service(api::routes::get_event);

        #[cfg(debug_assertions)]
            let app = app.route("api/v1/*", web::method(http::Method::OPTIONS).to(api::routes::cors_event));

            app
    }
    )
        .bind(format!("0.0.0.0:{}", &port))?
        .run()
        .await
}

