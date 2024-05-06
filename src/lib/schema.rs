// @generated automatically by Diesel CLI.

diesel::table! {
    card (id) {
        id -> Integer,
        src -> Text,
        tr -> Text,
        good -> Integer,
        bad -> Integer,
    }
}
