#![allow(unused)]
#![allow(clippy::all)]
#![allow(non_camel_case_types)]

use diesel_derive_enum::DbEnum;
use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds;
use serde::Serialize;

#[derive(DbEnum, Serialize, Debug)]
pub enum Event_type {
    Run,
    Trail,
    Cross,
    Triathlon,
    Ironman,
    Bike,
    Other,
}

#[derive(DbEnum, Serialize, Debug)]
pub enum Inscription_intent {
    Buy,
    Sell,
}

#[derive(DbEnum, Serialize, Debug)]
pub enum Gender {
    Man,
    Women
}

#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub event_type: Event_type,
    pub localisation: String,
    #[serde(with = "ts_seconds")]
    pub event_date: NaiveDateTime,
    pub event_link: String,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Inscription {
    pub id: i32,
    pub user_id: i32,
    pub event_id: i32,
    pub distance: String,
    pub price: f32,
    pub intent: Inscription_intent,
    #[serde(with = "ts_seconds")]
    pub created_at: NaiveDateTime,
    pub note: String,
    pub gender: Gender,
}

#[derive(Queryable, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub contact_1: Option<String>,
    pub contact_2: Option<String>,
    pub contact_3: Option<String>,
}

