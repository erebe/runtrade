#![allow(unused)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]

use diesel_derive_enum::DbEnum;
use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;
use serde::{ Serialize, Deserialize};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use super::schema::{events, users};

use lazy_static;
use uuid::Uuid;

lazy_static! {
    pub static ref EVENT_TYPES: Vec<Event_type> = Event_type::iter().collect();
    pub static ref INSCRIPTION_INTENTS: Vec<Inscription_intent> = Inscription_intent::iter().collect();
    pub static ref GENDERS: Vec<Gender> = Gender::iter().collect();
}

#[derive(DbEnum, Serialize, Deserialize, EnumIter, Debug)]
pub enum Event_type {
    Run,
    Trail,
    Cross,
    Triathlon,
    Ironman,
    Bike,
    Other,
}

#[derive(DbEnum, Serialize, Deserialize, EnumIter, Debug)]
pub enum Inscription_intent {
    Buy,
    Sell,
}

#[derive(DbEnum, Serialize, Deserialize, EnumIter, Debug)]
pub enum Gender {
    Man,
    Woman,
    Other
}

#[derive(Queryable, QueryableByName, Serialize, Debug)]
#[table_name="events"]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub event_type: Event_type,
    pub localisation: String,
    #[serde(with = "ts_seconds")]
    pub event_date: NaiveDateTime,
    pub event_link: String,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    pub user_id: Uuid
}

#[derive(Queryable, Serialize, Debug)]
pub struct Inscription {
    pub id: i32,
    pub user_id: Uuid,
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

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name="users"]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub contact: String,
    #[serde(with = "ts_seconds")]
    pub last_logged: NaiveDateTime,
}

