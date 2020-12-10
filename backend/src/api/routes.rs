use actix_web::{web, Error, HttpResponse, get, put};
use crate::db::DbPool;
use crate::db;

use std::borrow::Borrow;
use std::ops::Deref;
use crate::db::insertables::{NewEvent, NewInscription, NewUser};
use chrono::{NaiveDateTime, Utc};

pub async fn cors_event() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "http://localhost:8080")
        .header("Access-Control-Allow-Methods", "PUT, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .body(""))
}

#[put("/api/v1/event")]
pub async fn add_event(mut payload: web::Json<NewEvent>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        payload.name = payload.name.to_lowercase();
        db::operations::create_event(db.get().unwrap().borrow(), &payload)
    })
        .await
        .map(|event| HttpResponse::Ok().json(event))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}

#[get("/api/v1/event/types")]
pub async fn get_events_types() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json( db::model::EVENT_TYPES.deref()))
}

#[get("/api/v1/events/search/{name}")]
pub async fn find_events(path: web::Path<String>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::find_events(db.get().unwrap().borrow(), path.0.as_str())
    })
        .await
        .map(|events| HttpResponse::Ok().json(events))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}

#[get("/api/v1/event/{id}")]
pub async fn get_event(path: web::Path<i32>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::get_event(db.get().unwrap().borrow(), path.0)
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

#[put("/api/v1/user/{id}/contact")]
pub async fn update_user_contact(path: web::Path<i32>, payload: web::Json<String>,  db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        db::operations::update_user_contact(db.get().unwrap().borrow(), path.0, &payload)
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

#[put("/api/v1/user/logged")]
pub async fn put_user_logged(mut logged_user: web::Json<NewUser>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        logged_user.last_logged = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
        let conn = db.get().unwrap();
        db::operations::update_user_last_logged(&conn, &logged_user)
    })
        .await
        .map(|user| HttpResponse::Ok().json(user))
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

#[put("/api/v1/inscription")]
pub async fn add_inscription(mut payload: web::Json<NewInscription>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
        payload.created_at = NaiveDateTime::from_timestamp(Utc::now().timestamp(), 0);
        db::operations::upsert_inscription(db.get().unwrap().borrow(), &payload)
    })
        .await
        .map(|inscription| HttpResponse::Ok().json(inscription))
        .map_err(|err| HttpResponse::InternalServerError().body(err.to_string()))?;
    Ok(res)
}

#[cfg(test)]
mod test {
    use crate::db::insertables::NewEvent;
    use crate::db::model::Event_type;
    use chrono::NaiveDateTime;

    #[test]
    fn scratch_test() {
        let ev = NewEvent {
            name: "sdf".to_string(),
            event_type: Event_type::Run,
            localisation: "sdf".to_string(),
            event_date: NaiveDateTime::from_timestamp(1706313552, 0),
            event_link: "sdfsdf".to_string()
        };

        println!("{:?}", serde_json::to_string(&ev).unwrap());
    }
}