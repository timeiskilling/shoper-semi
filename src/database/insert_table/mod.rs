use diesel::{prelude::{Insertable, Queryable}, Selectable};
use rocket::serde::Serialize;
use rocket::form::FromForm;
#[derive(Queryable,Selectable)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}


#[derive(Insertable,FromForm,Debug,Clone)]
#[diesel(table_name = crate::schema::products)]
pub struct InsertProd<'a> {
    pub name : &'a str,
    pub price : i32,
    pub description: Option<&'a str>,
}