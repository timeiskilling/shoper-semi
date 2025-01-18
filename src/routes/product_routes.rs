use diesel::query_dsl::methods::FilterDsl;
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;


#[get("/product/<other_id>")]
pub async fn product_detalis(other_id : i32,pool: &State<DbPool>) -> Template {
    let result = products
        .filter(id.eq(other_id))
        .load::<Product>(&mut pool.get().expect("Failed to get connection"))
        .expect("Error loading items");

        let mut context = std::collections::HashMap::new();
        context.insert("items", result); 
    
        Template::render("index", &context)

}
