use rocket::Rocket;
use crate::autorization::*;
pub trait RoutesCombination {
    fn manege_routes(self) -> Self;
}


// impl RoutesCombination for Rocket {
//     fn manege_routes(&self) -> Self {
//         self.mount("/", routes![
            
//         ]
//     )
//     }
// }