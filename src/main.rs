#[macro_use] extern crate rocket;
use autorization::routes_user::login;
use autorization::sentinel::*;
use database::establish_pool;
pub mod database;
pub mod schema;
pub mod routes;
use database::sorting::{sort_products_by_category, sorted_by, DbConn};
use rocket::fs::FileServer;
use routes::catcher::RemoveInterestCohort;
use routes::product_routes::*;
use rocket_dyn_templates::Template;
use routes::home_page::*;
use routes::shopping_cart::cart::*;
mod autorization;
use autorization::routes_user::*;
#[launch]
async fn rocket() -> _ {
    let pool = establish_pool().await;
    rocket::build()
        .manage(pool)   
        .attach(NoCacheFairing) 
        .attach(Template::fairing())
        .attach(RemoveInterestCohort)
        .attach(DbConn::fairing())
        .mount("/", routes![list_of_item,picture,product,category,search_by_query,get_categories,insetable,add_product
                            ,sorted_by,sort_products_by_category,login,register,profile,logout])
        .mount("/static", FileServer::from("static"))
}   
