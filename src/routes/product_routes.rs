use diesel::RunQueryDsl;
use rocket::State;
use rocket_dyn_templates::{context, Template};
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;

#[get("/")]
pub async fn list_of_item(pool : &State<DbPool>) -> Template {
    let result = products
        .load::<Product>(&mut pool.get().expect("Failed to get connection"))
        .expect("Error loading items");

        let mut context = std::collections::HashMap::new();
        context.insert("items", result); // передаємо вектор елементів
    
        Template::render("index1", &context)
}
