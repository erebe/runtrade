use actix_web::{web, Error, HttpResponse, get, put};
use crate::db::DbPool;
use crate::db;

use std::borrow::Borrow;

#[get("/api/v1/events/{name}")]
pub async fn find_events(path: web::Path<String>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::find_events(db.get().unwrap().borrow(), path.0.as_str())
    })
        .await
        .map(|events| HttpResponse::Ok().json(events))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}

#[get("/api/v1/user/id/{id}")]
pub async fn get_user_by_id(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::get_user(db.get().unwrap().borrow(), path.0)
    })
        .await
        .map(|video| HttpResponse::Ok().json(video))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}

#[get("/api/v1/user/name/{name}")]
pub async fn get_user_by_name(path: web::Path<String>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::get_user_by_name(db.get().unwrap().borrow(), path.0.as_str())
    })
        .await
        .map(|video| HttpResponse::Ok().json(video))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}
#[get("/api/v1/inscriptions/event_id/{id}")]
pub async fn get_inscriptions_by_event_id(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::get_inscription_by_event_id(db.get().unwrap().borrow(), path.0)
    })
        .await
        .map(|inscription| HttpResponse::Ok().json(inscription))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}
