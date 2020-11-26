use chrono::NaiveDateTime;
use super::schema::{events, users, inscriptions};
use super::model::{Event_type, Inscription_intent};

#[derive(Insertable, AsChangeset, PartialEq, Debug)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub contact_1: &'a str,
    pub contact_2: &'a str,
    pub contact_3: &'a str,
}

#[derive(Insertable, AsChangeset, Debug)]
#[table_name="events"]
pub struct NewEvent<'a> {
    pub name: &'a str,
    pub event_type: &'a Event_type,
    pub localisation: &'a str,
    pub event_date: &'a NaiveDateTime,
    pub event_link: &'a str,
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
}
