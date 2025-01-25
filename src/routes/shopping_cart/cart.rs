

use crate::database::insert_table::Product;
use crate::database::insert_table::Category;
use diesel::prelude::*;
use rocket::serde::json::json;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::DbPool;
use crate::schema::products::dsl::*;
use rocket::serde::json::Json;


#[get("/category/<ids>")]
pub async fn category(pool: &State<DbPool>, ids: i32) -> Template {
    let mut conn = pool.get().unwrap();
    let mut conn2 = pool.get().unwrap();
    
    let category_name_result = tokio::task::spawn_blocking(move || {
        use crate::schema::categories::dsl::*;
        categories.find(ids).select(name).first::<String>(&mut conn).unwrap()
    }).await;

    
    let category_name = match category_name_result {
        Ok(names) => names,
        Err(_) => return Template::render("error", &json!({"error": "Failed to load category name"})),
    };

    
    let products_result = tokio::task::spawn_blocking(move || {
        products.filter(category_id.eq(ids)).load::<Product>(&mut conn2).unwrap()
    }).await;

    match products_result {
        Ok(product_list) => {
            let context = json!({
                "category_name": category_name,
                "items": product_list
            });
            Template::render("categories", &context)
        },
        Err(_) => Template::render("error", &json!({"error": "Failed to load products"})),
    }
}


#[get("/api/categories")]
pub async fn get_categories(pool: &State<DbPool>) -> Json<Vec<Category>> {
    let mut conn = pool.get().unwrap();
    let result = tokio::task::spawn_blocking(move || {
        use crate::schema::categories::dsl::*;
        categories.load::<Category>(&mut conn).unwrap()
    }).await;

    result.map(Json).unwrap_or_else(|_| Json(Vec::new()))
}