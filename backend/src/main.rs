#[macro_use] extern crate diesel;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate log;
extern crate dotenv;

mod db;
mod api;

use crate::db::get_postgres_connection_pool;
use std::env;
use actix_web::{HttpServer, App, middleware, web, http};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv::dotenv().ok();
    // Logging
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    env_logger::init();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let port = std::env::var("PORT")
        .unwrap_or("8080".to_string());
    let webapp_dir_path = std::env::var("WEBAPP_DIR_PATH")
        .unwrap_or("/home/erebe/progs/runtrade/webapp/dist".to_string());

    info!("PORT : {}", port);
    // info!("DATABASE_URL: {}", database_url);
    info!("JWK_E : {}", api::auth::JWK_E.as_str());
    info!("JWK_N : {}", api::auth::JWK_N.as_str());

    let db_pool = get_postgres_connection_pool(&database_url)
        .expect("Cannot create postgres connection pool");


    HttpServer::new(move || {
        let mut headers = middleware::DefaultHeaders::new();
        #[cfg(debug_assertions)] // If in not release allow localhost to ease developpement
        (headers = headers.header("Access-Control-Allow-Origin", "http://localhost:8080"));

        let app = App::new()
            .wrap(headers)
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
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
            .service(api::routes::delete_inscription)
            .service(api::routes::get_event)
            .service(fs::Files::new("/", &webapp_dir_path)
               .index_file("index.html")
                .prefer_utf8(true)
            );

        #[cfg(debug_assertions)]
            let app = app.route("api/v1/*", web::method(http::Method::OPTIONS).to(api::routes::cors_event));

            app
    }
    )
        .bind(format!("0.0.0.0:{}", &port))?
        .run()
        .await
}

