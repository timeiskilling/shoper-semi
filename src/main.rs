#[macro_use] extern crate rocket;
use database::establish_pool;
pub mod database;
pub mod schema;
pub mod routes;
use rocket::fs::FileServer;
use routes::product_routes::*;
use rocket_dyn_templates::Template;
use routes::home_page::*;
extern crate rocket_dyn_templates;
use routes::shopping_cart::cart::*;


#[launch]
async fn rocket() -> _ {
    let pool = establish_pool().await;
    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount("/", routes![list_of_item,picture,product_details,take_category,search_by_query])
        .mount("/static", FileServer::from("static"))
}
