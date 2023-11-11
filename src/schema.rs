// @generated automatically by Diesel CLI.

diesel::table! {
    liquidations (id) {
        id -> Int4,
        symbol -> Varchar,
        side -> Varchar,
        quantity -> Numeric,
        created_at -> Timestamp,
    }
}
