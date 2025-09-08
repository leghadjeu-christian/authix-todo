// @generated automatically by Diesel CLI.

diesel::table! {
    to_do (id) {
        id -> Int4,
        title -> Varchar,
        status -> Varchar,
        user_id -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Varchar,
        email -> Varchar,
        password -> Varchar,
    }
}

diesel::joinable!(to_do -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    to_do,
    users,
);
