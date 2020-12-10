use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;
use serde::{ Deserialize };
use super::schema::{events, users, inscriptions};
use super::model::{Event_type, Inscription_intent};
use crate::db::model::Gender;
use uuid::Uuid;

#[derive(Insertable, AsChangeset, Deserialize, PartialEq, Debug)]
#[table_name="users"]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub contact: String,
    pub external_id: Uuid,
    #[serde(with = "ts_seconds")]
    pub last_logged: NaiveDateTime
}

#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name="events"]
pub struct NewEvent {
    pub name: String,
    pub event_type: Event_type,
    pub localisation: String,
    #[serde(with = "ts_seconds")]
    pub event_date: NaiveDateTime,
    pub event_link: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    pub user_id: i32,
}
#[derive(Insertable, AsChangeset, Deserialize, Debug)]
#[table_name="inscriptions"]
pub struct NewInscription {
    pub user_id: i32,
    pub event_id: i32,
    pub category: String,
    pub price: f32,
    pub currency: String,
    pub intent: Inscription_intent,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    pub note: String,
    pub gender: Gender,
}
