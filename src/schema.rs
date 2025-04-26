// @generated automatically by Diesel CLI.

diesel::table! {
    incidents (id) {
        id -> Integer,
        title -> Text,
        description -> Text,
        severity -> Text,
        reported_at -> Timestamp,
    }
}
