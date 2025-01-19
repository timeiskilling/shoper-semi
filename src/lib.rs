// use database::establish_pool;

// pub mod database;
// pub mod schema;
// mod routes;
// use rocket_dyn_templates::Template;
// use routes::product_routes::*;

// #[macro_use]
// extern crate rocket;

// #[launch]
// fn rocket() -> _ {
//     let pool = establish_pool();
//     rocket::build()
//         .manage(pool)
//         .mount("/", routes![])
//         .attach(Template::fairing())
        
// }

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
