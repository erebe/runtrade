use actix_web::{web, Error, HttpResponse, get, put};
use crate::db::DbPool;
use crate::db;

use std::borrow::Borrow;
use std::ops::Deref;
use crate::db::insertables::NewEvent;

pub async fn cors_event() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .header("Access-Control-Allow-Origin", "http://localhost:8080")
        .header("Access-Control-Allow-Methods", "PUT, GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "Content-Type, Authorization")
        .body(""))
}

#[put("/api/v1/event")]
pub async fn add_event(payload: web::Json<NewEvent>, db: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let res = web::block(move || {
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