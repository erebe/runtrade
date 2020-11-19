#![allow(unused)]

use crate::db::insertables::*;

use diesel::prelude::*;
use diesel::{PgConnection, QueryResult};
use crate::db::model::{User, Event};
use crate::schema::users::dsl::{users, id};
use crate::schema::events::dsl::events;

//
// USERS
//
pub fn get_user(conn: &PgConnection, user_id: i32) -> QueryResult<User> {
    use crate::schema::users::id;

    users.filter(id.eq(user_id))
        .first::<User>(conn)
}

pub fn get_user_by_name(conn: &PgConnection, user_name: &str) -> QueryResult<User> {
    use crate::schema::users::name;

    users.filter(name.eq(user_name))
        .first::<User>(conn)
}

pub fn create_user(conn: &PgConnection, user: &NewUser) -> QueryResult<User> {
    diesel::insert_into(users)
        .values(user)
        .get_result(conn)
}

pub fn update_user(conn: &PgConnection, user_id: i32,  user: &NewUser) -> QueryResult<User> {
    use crate::schema::users::id;

    diesel::update(users.filter(id.eq(user_id)))
        .set(user)
        .get_result(conn)
}

pub fn delete_user(conn: &PgConnection, user_id: i32) -> QueryResult<usize> {
    use crate::schema::users::id;

    diesel::delete(users.filter(id.eq(user_id)))
        .execute(conn)
}

//
// EVENTS
//
pub fn create_event(conn: &PgConnection, event: &NewEvent) -> QueryResult<Event> {
    use crate::schema::events::id;

    diesel::insert_into(events)
        .values(event)
        .get_result(conn)
}

pub fn update_event(conn: &PgConnection, event_id: i32, event: &NewEvent) -> QueryResult<Event> {
    use crate::schema::events::id;

    diesel::update(events.filter(id.eq(event_id)))
        .set(event)
        .get_result(conn)
}

pub fn delete_event(conn: &PgConnection, event_id: i32) -> QueryResult<usize> {
    use crate::schema::events::id;

    diesel::delete(events.filter(id.eq(event_id)))
        .execute(conn)
}


//
// INSCRIPTION
//


#[cfg(feature = "functional_tests")]
#[cfg(test)]
mod test {
    use crate::db::operations::{create_user, update_user};
    use crate::db::insertables::NewUser;
    use diesel::{PgConnection, Connection};
    use std::env;
    use dotenv::dotenv;

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
            contact_3: "tete"
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
            contact_3: "tete"
        };
        let user_id = create_user(&conn, &user).unwrap().id;
        user.name = "abcdefg";

        let updated_user = update_user(&conn, user_id, &user).unwrap();
        assert_eq!(updated_user.name, user.name);
        assert_ne!(updated_user.name, "toto");
        assert_eq!(updated_user.email, user.email);
    }
}