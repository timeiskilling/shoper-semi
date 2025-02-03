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
    product_images (id) {
        id -> Int4,
        product_id -> Nullable<Int4>,
        file_path -> Varchar,
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

diesel::table! {
    tokens (id) {
        id -> Int4,
        #[max_length = 255]
        token -> Varchar,
        user_id -> Int4,
        #[max_length = 50]
        token_type -> Varchar,
        issued_at -> Timestamp,
        expires_at -> Timestamp,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        #[max_length = 300]
        password_hash -> Varchar,
        #[max_length = 50]
        role -> Varchar,
    }
}

diesel::joinable!(product_images -> products (product_id));
diesel::joinable!(products -> categories (category_id));
diesel::joinable!(tokens -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    product_images,
    products,
    tokens,
    users,
);
