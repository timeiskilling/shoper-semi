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
use rocket::http::CookieJar;

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



#[derive(serde::Serialize ,Clone)]
struct IndexContext {
    items: Vec<Product>,
    username: Option<String>,
    query_bames : String,
}

#[derive(serde::Serialize ,Clone)]
struct NotFound {
    username: Option<String>,
    query : String,
}


#[get("/search?<query>")]
pub async fn search_by_query(pool: &State<DbPool>, query: String, cookies: &CookieJar<'_>,) -> Template {

    let username = cookies
        .get_private("username")
        .map(|cookie| cookie.value().to_string());

    let username_clone = username.clone();

    let queru_one = query.clone();
    let query_bames = query.clone();
    let mut conn  = pool.get().unwrap();

    let result = spawn_blocking(move || {
        products.filter(lower(name).like(format!("%{}%", query.to_lowercase())))
        .load::<Product>(&mut conn)
        .unwrap()
    })
    .await;

    let tepm = IndexContext { 
        items: result.unwrap(), 
        username, 
        query_bames 
    };
    
    let temp1 = NotFound { 
        username : username_clone.clone(), 
        query : queru_one
    };

    
    if let vecs = tepm.clone() {
        if vecs.items.is_empty() {

            return Template::render("serch_not_found", &temp1);
        }

        return Template::render("search_page", &tepm);
    }


    Template::render("error", &tepm)
}