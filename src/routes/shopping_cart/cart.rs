use crate::database::insert_table::Product;
use diesel::prelude::*;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::DbPool;
use crate::schema::products::dsl::*;

#[get("/category/<ids>")]
pub async fn take_category(pool : &State<DbPool>, ids : i32 ) -> Template {
    let mut conn = pool.get().unwrap();
    let result = tokio::task::spawn_blocking(move || {
        products
            .filter(category_id.eq(ids))
            .load::<Product>(&mut conn)
            .unwrap()
    }).await;

    if let Ok(vec) = result {
        let mut context = std::collections::HashMap::new();
        context.insert("items", vec);

       
        match ids {
            1 => Template::render("electro", &context),
            2 => Template::render("clothing", &context),
            _ => Template::render("default_category", &context)
        }
    } else {
        let mut context = std::collections::HashMap::new();
        context.insert("error", "Failed to load product details");

        Template::render("error", &context)
    }
}
