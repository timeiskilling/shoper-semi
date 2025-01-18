// @generated automatically by Diesel CLI.

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        price -> Int4,
        description -> Nullable<Text>,
        file_path -> Varchar,
    }
}
