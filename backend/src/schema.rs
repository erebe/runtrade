#![allow(unused)]

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping};

    events (id) {
        id -> Int4,
        name -> Text,
        event_type -> Event_typeMapping,
        localisation -> Text,
        event_date -> Timestamp,
        event_link -> Text,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping};

    inscriptions (id) {
        id -> Int4,
        user_id -> Int4,
        event_id -> Int4,
        distance -> Text,
        price -> Float4,
        intent -> Inscription_intentMapping,
        created_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping};

    users (id) {
        id -> Int4,
        name -> Text,
        email -> Text,
        contact_1 -> Nullable<Text>,
        contact_2 -> Nullable<Text>,
        contact_3 -> Nullable<Text>,
    }
}

joinable!(inscriptions -> events (event_id));
joinable!(inscriptions -> users (user_id));

allow_tables_to_appear_in_same_query!(
    events,
    inscriptions,
    users,
);
