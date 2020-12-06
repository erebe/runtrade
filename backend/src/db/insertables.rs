use chrono::NaiveDateTime;
use serde::{ Serialize, Deserialize };
use super::schema::{events, users, inscriptions};
use super::model::{Event_type, Inscription_intent};
use crate::db::model::Gender;

#[derive(Insertable, AsChangeset, PartialEq, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub contact_1: &'a str,
    pub contact_2: &'a str,
    pub contact_3: &'a str,
}

#[derive(Insertable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name="events"]
pub struct NewEvent {
    pub name: String,
    #[serde(alias = "type")]
    pub event_type: Event_type,
    pub localisation: String,
    #[serde(alias = "date")]
    pub event_date: NaiveDateTime,
    #[serde(alias = "link")]
    pub event_link: String,
}
#[derive(Insertable, AsChangeset, Debug)]
#[table_name="inscriptions"]
pub struct NewInscriptions<'a> {
    pub user_id: i32,
    pub event_id: i32,
    pub distance: &'a str,
    pub price: f32,
    pub intent: &'a Inscription_intent,
    pub created_at: &'a NaiveDateTime,
    pub note: &'a str,
    pub gender: &'a Gender,
}
