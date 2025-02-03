use diesel::{prelude::{Insertable, Queryable}, Selectable};
use rocket::form::FromForm;
use diesel::prelude::*;
#[derive(Queryable, Selectable, serde::Serialize,Clone)]
#[diesel(table_name = crate::schema::products)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub description: Option<String>,
    pub file_path : String,
    pub category_id : Option<i32>,
}


#[derive(Insertable,FromForm,Debug,Clone)]
#[diesel(table_name = crate::schema::products)]
pub struct InsertProd<'a> {
    pub name : &'a str,
    pub price : i32,
    pub description: Option<&'a str>,
    pub file_path : String,
    pub category_id : Option<i32>,
}


#[derive(Queryable, Identifiable, Associations, Debug,serde::Serialize,Clone)]
#[diesel(table_name = crate::schema::categories)]
#[diesel(belongs_to(Category, foreign_key = parent_id))] 
pub struct Category {
    pub id: i32,
    pub name: String,
    pub parent_id: Option<i32>, 
}

#[derive(Queryable, Identifiable, Associations, serde::Serialize)]
#[diesel(table_name = crate::schema::product_images)]
#[diesel(belongs_to(Product))]
pub struct ProductImage {
    pub id: i32,
    pub product_id: Option<i32>,
    pub file_path: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::product_images)]
pub struct NewProductImage {
    pub product_id: i32,
    pub file_path: String,
}

#[derive(Queryable, Identifiable,Debug,serde::Serialize,Clone)]
#[diesel(table_name = crate::schema::users)]
pub struct UserEx {
    pub id : i32,
    pub username : String,
    pub password_hash : String,
    pub role : String,
}


#[derive(Insertable, serde::Serialize, serde::Deserialize)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub username: String,
    pub password_hash: String,
    pub role: String,
}
