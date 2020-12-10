#![allow(unused_imports)]

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    events (id) {
        id -> Int4,
        name -> Text,
        event_type -> Event_typeMapping,
        localisation -> Text,
        event_date -> Timestamp,
        event_link -> Text,
        created_at -> Timestamp,
        user_id -> Int4,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    inscriptions (id) {
        id -> Int4,
        user_id -> Int4,
        event_id -> Int4,
        category -> Text,
        price -> Float4,
        currency -> Varchar,
        intent -> Inscription_intentMapping,
        created_at -> Timestamp,
        note -> Text,
        gender -> GenderMapping,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        contact -> Text,
        external_id -> Uuid,
        last_logged -> Timestamp,
    }
}

joinable!(events -> users (user_id));
joinable!(inscriptions -> events (event_id));
joinable!(inscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    inscriptions,
    users,
);
