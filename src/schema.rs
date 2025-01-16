// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        price -> Numeric,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}
