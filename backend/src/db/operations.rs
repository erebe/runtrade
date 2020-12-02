#![allow(unused)]

use chrono::{NaiveDateTime, NaiveTime, Utc};
use diesel::{PgConnection, QueryResult};
use diesel::prelude::*;

use super::insertables::*;
use super::model::{Event, User};
use super::schema::events::columns::{event_date, event_link, localisation, name};
use super::schema::events::dsl::events;
use super::schema::users::dsl::{id, users};
use crate::db::model::Inscription;
use chrono::format::Numeric::Timestamp;

const MAX_RESULTS: i64 = 200;

//
// USERS
//
pub fn get_user(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
    use super::schema::users::id;

    users.filter(id.eq(user_id))
        .first::<User>(conn)
}

pub fn get_user_by_name(conn: &PgConnection, user_name: &str) -> QueryResult<User> {
    use super::schema::users::name;

    users.filter(name.eq(user_name))
        .first::<User>(conn)
}

pub fn create_user(conn: &PgConnection, user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(user)
        .get_result(conn)
}

pub fn update_user(conn: &PgConnection, user_id: i32, user: &NewUser) -> QueryResult<User> {
    use super::schema::users::id;

    diesel::update(users.filter(id.eq(user_id)))
        .set(user)
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> QueryResult<usize> {
    use super::schema::users::id;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
}

//
// EVENTS
//
pub fn find_events(conn: &PgConnection, event: &str) -> QueryResult<Vec<Event>> {
    use super::schema::events::id;

    let search_pat = format!("%{}%", event.replace(' ', "%"));
    let today = Utc::today().naive_utc();
    let midnight = NaiveTime::from_hms(0, 0, 0);

    events
        .filter(name.ilike(&search_pat)
            .or(localisation.ilike(&search_pat))
            .or(event_link.ilike(&search_pat))
            .and(event_date.gt(NaiveDateTime::new(today, midnight))))
        .limit(MAX_RESULTS)
        .load::<Event>(conn)
}

pub fn create_event(conn: &PgConnection, event: &NewEvent) -> QueryResult<Event> {
    use super::schema::events::id;

    diesel::insert_into(events)
        .values(event)
        .get_result(conn)
}

pub fn update_event(conn: &PgConnection, event_id: i32, event: &NewEvent) -> QueryResult<Event> {
    use super::schema::events::id;

    diesel::update(events.filter(id.eq(event_id)))
        .set(event)
        .get_result(conn)
}

pub fn delete_event(conn: &PgConnection, event_id: i32) -> QueryResult<usize> {
    use super::schema::events::id;

    diesel::delete(events.filter(id.eq(event_id)))
        .execute(conn)
}


//
// INSCRIPTION
//
pub fn get_inscription_by_event_id(conn: &PgConnection, event_ids: i32) -> QueryResult<Vec<(Inscription, User, Event)>> {
    use super::schema::inscriptions::dsl::*;

    inscriptions.filter(event_id.eq(&event_ids))
        .inner_join(users)
        .inner_join(events)
        .load::<(Inscription, User, Event)>(conn)
}
pub fn upsert_inscription(conn: &PgConnection, inscription: &NewInscriptions) -> QueryResult<Inscription> {
    use super::schema::inscriptions::dsl::*;

    diesel::insert_into(inscriptions)
        .values(inscription)
        .on_conflict(id)
        .do_update()
        .set(created_at.eq(Utc::now().naive_utc()))
        .get_result(conn)
}

#[cfg(feature = "functional_tests")]
#[cfg(test)]
mod test {
    use std::env;

    use chrono::NaiveDateTime;
    use diesel::{Connection, PgConnection, QueryResult, ExpressionMethods, RunQueryDsl};
    use dotenv::dotenv;

    use crate::db::insertables::{NewEvent, NewUser, NewInscriptions};
    use crate::db::model::{Event, Event_type, Inscription_intent, Gender};
    use crate::db::operations::{create_event, create_user, find_events, update_user, upsert_inscription};
    use crate::db::schema::events::dsl::events;
    use crate::db::schema::events::columns::event_date;

    fn establish_connection() -> PgConnection {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    #[test]
    fn test_create_user() {
        let conn = establish_connection();
        let user = NewUser {
            name: "toto",
            email: "tata",
            contact_1: "tutu",
            contact_2: "titi",
            contact_3: "tete",
        };

        assert_eq!(create_user(&conn, &user).unwrap().name, user.name);
    }

    #[test]
    fn test_update_user() {
        let conn = establish_connection();

        let mut user = NewUser {
            name: "toto",
            email: "tata",
            contact_1: "tutu",
            contact_2: "titi",
            contact_3: "tete",
        };
        let user_id = create_user(&conn, &user).unwrap().id;
        user.name = "abcdefg";

        let updated_user = update_user(&conn, user_id, &user).unwrap();
        assert_eq!(updated_user.name, user.name);
        assert_ne!(updated_user.name, "toto");
        assert_eq!(updated_user.email, user.email);
    }


    //#[test]
    fn test_find_events() {
        let conn = establish_connection();

        // for ix in 1..200 {
        //     let nam = format!("toto{}", ix);
        //     let mut event = NewEvent {
        //         name: nam.as_str(),
        //         event_type: &Event_type::Run,
        //         localisation: "Paris",
        //         event_date: &NaiveDateTime::from_timestamp(1606313552, 0),
        //         event_link: "https://google.com",
        //     };
        //     let ev = create_event(&conn, &event).unwrap();
        // };

        diesel::update(events).set(event_date.eq(NaiveDateTime::from_timestamp(1706313552, 0)))
            .execute(&conn);

        //let events: QueryResult<Vec<Event>> = find_events(&conn, "tot");
        //assert_eq!(events.unwrap().len(), 1);
    }
    
    #[test]
    fn create_fake_date() {
        let user1 = NewUser {
            name: "erebe",
            email: "erebe@erebe.eu",
            contact_1: "facebook: https://www.facebook.com/erebe.dellu.16",
            contact_2: "email: nemesia.lilith@gmail.com",
            contact_3: "telephone: 336597126xx"
        };
        let user2 = NewUser {
            name: "Romain Gerard",
            email: "romain.gerard@erebe.eu",
            contact_1: "facebook: https://www.facebook.com/erebe.dellu.16",
            contact_2: "email: nemesia.lilith@gmail.com",
            contact_3: "telephone: 336597126xx"
        };
        let conn = establish_connection();
        let us1 = create_user(&conn, &user1).unwrap();
        let us2 = create_user(&conn, &user2).unwrap();

        let event1 = NewEvent {
            name: "Marathon Valence",
            event_type: &Event_type::Run,
            localisation: "Valence Spain",
            event_date: &NaiveDateTime::from_timestamp(1706313552, 0),
            event_link: "https://www.valenciaciudaddelrunning.com/fr/marathon_fr/42k/"
        };
        let event2 = NewEvent {
            name: "Semi de Deauville",
            event_type: &Event_type::Run,
            localisation: "Deauville",
            event_date: &NaiveDateTime::from_timestamp(1706313552, 0),
            event_link: "https://www.marathondeauville.fr/"
        };
        let event3 = NewEvent {
            name: "Blagnac",
            event_type: &Event_type::Run,
            localisation: "Blagnac",
            event_date: &NaiveDateTime::from_timestamp(1615109893, 0),
            event_link: "http://www.semi-blagnac.com/pages/index.php"
        };
        let ev1 = create_event(&conn, &event1).unwrap();
        let ev2 = create_event(&conn, &event2).unwrap();
        let ev3 = create_event(&conn, &event3).unwrap();
        
        let inscription1 = NewInscriptions {
            user_id: us1.id,
            event_id: ev1.id,
            distance: "42km",
            price: 64.0,
            intent: &Inscription_intent::Buy,
            created_at: &NaiveDateTime::from_timestamp(1606815825, 0),
            note: "42Km c'est trop long",
            gender: &Gender::Women,
        };
        let inscription2 = NewInscriptions {
            user_id: us2.id,
            event_id: ev2.id,
            distance: "21km",
            price: 23.0,
            intent: &Inscription_intent::Sell,
            created_at: &NaiveDateTime::from_timestamp(1606815825, 0),
            note: "Blessure genoux",
            gender: &Gender::Man,
        };
        let inscription3 = NewInscriptions {
            user_id: us1.id,
            event_id: ev1.id,
            distance: "10km",
            price: 30.0,
            intent: &Inscription_intent::Buy,
            created_at: &NaiveDateTime::from_timestamp(1606815825, 0),
            note: "je veux le faire parceque courrir c'est trop bien pour ton corps de grande malade. Vive le F1",
            gender: &Gender::Women,
        };
        let inscription4 = NewInscriptions {
            user_id: us1.id,
            event_id: ev1.id,
            distance: "21km",
            price: 30.0,
            intent: &Inscription_intent::Sell,
            created_at: &NaiveDateTime::from_timestamp(1606815825, 0),
            note: "Il pleut",
            gender: &Gender::Women,
        };

        upsert_inscription(&conn, &inscription1).unwrap();
        upsert_inscription(&conn, &inscription2).unwrap();
        upsert_inscription(&conn, &inscription3).unwrap();
        upsert_inscription(&conn, &inscription4).unwrap();
    }
}