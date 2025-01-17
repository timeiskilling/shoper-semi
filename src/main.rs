#[macro_use] extern crate rocket;
use database::establish_pool;

pub mod database;
pub mod schema;
mod routes;
use rocket_dyn_templates::Template;
use routes::product_routes::*;
extern crate rocket_dyn_templates;


#[launch]
fn rocket() -> _ {
    let pool = establish_pool();
    rocket::build()
        .manage(pool)
        .attach(Template::fairing())
        .mount("/", routes![list_of_item])
}
