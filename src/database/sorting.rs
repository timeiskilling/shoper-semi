use diesel::{ExpressionMethods, QueryDsl};
use diesel::RunQueryDsl;

use rocket::serde::json::Json;
use rocket_sync_db_pools::database;


use super::insert_table::Product;


#[database("pg_db")]
pub struct DbConn(diesel::PgConnection);


#[get("/sort?<how_sort>")]
pub async fn sorted_by(conn: DbConn, how_sort: Option<String>) -> Json<Vec<Product>> {
    let result = conn.run(move |c| {
        use crate::schema::products::dsl::*;

        let mut query = products.into_boxed();

        if let Some(sort_option) = how_sort {
            query = match sort_option.as_str() {
                "price_asc" => query.order(price.asc()),
                "price_desc" => query.order(price.desc()),
                "name_asc" => query.order(name.asc()),
                "name_desc" => query.order(name.desc()),
                _ => query,
            };
        }
        query.load::<Product>(c)
    }).await;

    match result {
        Ok(products) => Json(products),
        Err(e) => {
            eprintln!("Error loading products: {}", e);
            Json(vec![])
        }
    }
}


#[get("/category/<category_ids>/sort?<how_sort>")]
pub async fn sort_products_by_category(conn: DbConn, category_ids: i32, how_sort: Option<String>) -> Json<Vec<Product>> {
    let result = conn.run(move |c| {
        use crate::schema::products::dsl::*;

        let mut query = products.filter(category_id.eq(category_ids)).into_boxed();

        if let Some(sort_option) = how_sort {
            query = match sort_option.as_str() {
                "price_asc" => query.order(price.asc()),
                "price_desc" => query.order(price.desc()),
                "name_asc" => query.order(name.asc()),
                "name_desc" => query.order(name.desc()),
                _ => query,
            };
        }
        query.load::<Product>(c)
    }).await;

    match result {
        Ok(products) => Json(products),
        Err(e) => {
            eprintln!("Error loading products: {}", e);
            Json(vec![])
        }
    }
}