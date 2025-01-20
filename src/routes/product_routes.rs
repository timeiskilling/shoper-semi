use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::State;
use rocket_dyn_templates::Template;
use tokio::task::spawn_blocking;
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;


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

