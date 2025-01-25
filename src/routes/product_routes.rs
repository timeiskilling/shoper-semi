use diesel::query_dsl::methods::{FilterDsl, FindDsl};
use diesel::{define_sql_function,ExpressionMethods, RunQueryDsl, TextExpressionMethods};
use rocket::State;
use rocket_dyn_templates::Template;
use tokio::task::spawn_blocking;
use crate::database::insert_table::{Product, ProductImage};
use crate::database::DbPool;
use crate::schema::products::dsl::*;
use crate::schema::product_images::dsl::*;
use rocket::serde::json::json;
use diesel::sql_types::Text;

define_sql_function!(fn lower(x: Text) -> Text);

#[get("/product/<ids>")]
pub async fn product(pool: &State<DbPool>, ids: i32) -> Template {
    let mut conn = pool.get().unwrap();
    let mut conn2 = pool.get().unwrap();
    let product_result = tokio::task::spawn_blocking(move || {
        products.find(ids).first::<Product>(&mut conn).unwrap()
    }).await;

    let images_result = tokio::task::spawn_blocking(move || {
        product_images.filter(product_id.eq(&ids)).load::<ProductImage>(&mut conn2).unwrap()
    }).await;

    match (product_result, images_result) {
        (Ok(product), Ok(image_list)) => {
            let context = json!({
                "product": product,
                "images": image_list
            });
            Template::render("index", &context)
        },
        _ => Template::render("error", &json!({"error": "Failed to load product or images"})),
    }
}

#[get("/search?<query>")]
pub async fn search_by_query(pool: &State<DbPool>, query: String) -> Template {
    let query_bames = query.clone();
    let mut conn  = pool.get().unwrap();

    let result = spawn_blocking(move || {
        products.filter(lower(name).like(format!("%{}%", query.to_lowercase())))
        .load::<Product>(&mut conn)
        .unwrap()
    })
    .await;


    let mut context = std::collections::HashMap::new();
    
    if let Ok(vecs) = result {
        if vecs.is_empty() {
            context.insert("error", "Нічого не знайдено за вашим запитом");
            context.insert("query", &query_bames);

            return Template::render("serch_not_found", &context);
        }
        let mut context = std::collections::HashMap::new();
        context.insert("items", vecs);
        return Template::render("search_page", &context);
    }

    context.insert("error", "Не вдалося завантажити інформацію про товари");
    Template::render("error", &context)
}