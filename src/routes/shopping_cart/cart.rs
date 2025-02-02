
use crate::database::insert_table::Product;
use crate::database::insert_table::Category;
use diesel::prelude::*;
use rocket::serde::json::json;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::DbPool;

use rocket::serde::json::Json;
use crate::schema::categories::dsl::*;

fn get_all_subcategories(conn: &mut PgConnection, category_id: i32) -> Vec<i32> {
    let mut all_ids = vec![category_id];

    
    let subcategories: Vec<i32> = categories
        .filter(parent_id.eq(category_id))
        .select(id)
        .load::<i32>(conn) 
        .unwrap();

    
    for sub_id in subcategories {
        let sub_ids = get_all_subcategories(conn, sub_id);
        all_ids.extend(sub_ids);
    }

    all_ids
}


#[get("/category/<ids>")]
pub async fn category(pool: &State<DbPool>, ids: i32) -> Template {
    use crate::schema::products::dsl::*;
    let mut conn = pool.get().unwrap();
    let mut conn1 =pool.get().unwrap();
    let mut conn2 =pool.get().unwrap();
    
    let all_category_ids = tokio::task::spawn_blocking(move || {
        get_all_subcategories(&mut conn, ids)
    }).await.unwrap();

    let category_name_result = tokio::task::spawn_blocking(move || {
        use crate::schema::categories::dsl::*;
        categories.find(ids).select(name).first::<String>(&mut conn1).unwrap()
    }).await;

    let category_name = match category_name_result {
        Ok(names) => names,
        Err(_) => return Template::render("error", &json!({"error": "Failed to load category name"})),
    };

    let products_result = tokio::task::spawn_blocking(move || {
        products.filter(category_id.eq_any(&all_category_ids)).load::<Product>(&mut conn2).unwrap()
    }).await;

    match products_result {
        Ok(product_list) => {
            let context = json!({
                "category_name": category_name,
                "items": product_list,
                "category_id": ids
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