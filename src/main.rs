#[macro_use] extern crate rocket;
use autorization::sentinel::*;
use database::establish_pool;
pub mod database;
pub mod schema;
pub mod routes;
use rocket::fs::FileServer;
use routes::product_routes::*;
use rocket_dyn_templates::Template;
use routes::home_page::*;
use routes::shopping_cart::cart::*;
mod autorization;

#[launch]
async fn rocket() -> _ {
    let pool = establish_pool().await;
    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount("/", routes![list_of_item,picture,product,category,search_by_query,get_categories,insetable,add_product])
        .mount("/static", FileServer::from("static"))
}
