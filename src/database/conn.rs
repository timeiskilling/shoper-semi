use crate::database::insert_table::InsertProd;
use crate::schema::products::dsl::*;
use crate::database::DbPool;
use diesel::RunQueryDsl;
use rocket::tokio::task::spawn_blocking;

pub async fn insert_into_database_product<'a>(pool: DbPool, product: InsertProd<'static>)  {
    let some_prod = Box::new(product);
     
    let result = spawn_blocking(move || {
        diesel::insert_into(products)
            .values(&*some_prod) 
            .execute(&mut pool.get().expect("err"))
    }).await;
}



