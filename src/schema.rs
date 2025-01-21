// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        parent_id -> Nullable<Int4>,
    }
}

diesel::table! {
    products (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        price -> Int4,
        description -> Nullable<Text>,
        file_path -> Varchar,
        category_id -> Nullable<Int4>,
    }
}

diesel::joinable!(products -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    products,
);
