use diesel::query_dsl::methods::FilterDsl;
use diesel::{define_sql_function,ExpressionMethods, RunQueryDsl, TextExpressionMethods};
use rocket::State;
use rocket_dyn_templates::Template;
use tokio::task::spawn_blocking;
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;

use diesel::dsl::sql;
use diesel::sql_types::Text;

define_sql_function!(fn lower(x: Text) -> Text);

#[get("/product/<other_id>")]
pub async fn product_details(other_id: i32, pool: &State<DbPool>) -> Template {
    let mut conn = pool.get().unwrap();
    let result = spawn_blocking(move || {
        products
            .filter(id.eq(other_id))
            .load::<Product>(&mut conn)
            .unwrap()
    })
    .await;

    if let Ok(vec) = result {
        let mut context = std::collections::HashMap::new();
        context.insert("items", vec);

        Template::render("index", &context)
    } else {
        
        let mut context = std::collections::HashMap::new();
        context.insert("error", "Failed to load product details");

        Template::render("error", &context)
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
