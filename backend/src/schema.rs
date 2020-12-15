table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    events (id) {
        id -> Int4,
        name -> Text,
        event_type -> Event_type,
        localisation -> Text,
        event_date -> Timestamp,
        event_link -> Text,
        created_at -> Timestamp,
        user_id -> Nullable<Uuid>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    inscriptions (id) {
        id -> Int4,
        user_id -> Nullable<Uuid>,
        event_id -> Int4,
        category -> Text,
        price -> Float4,
        currency -> Varchar,
        intent -> Inscription_intent,
        created_at -> Timestamp,
        note -> Text,
        gender -> Gender,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::db::model::{Event_typeMapping, Inscription_intentMapping, GenderMapping};

    users (id) {
        id -> Uuid,
        name -> Text,
        email -> Text,
        contact -> Text,
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
