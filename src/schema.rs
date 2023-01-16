// @generated automatically by Diesel CLI.

diesel::table! {
    posts (id) {
        id -> Integer,
        title -> Varchar,
        author -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
