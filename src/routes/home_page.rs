use std::path::{Path, PathBuf};
use diesel::RunQueryDsl;
use rocket::fs::NamedFile;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::insert_table::{InsertProd, Product};
use crate::database::DbPool;
use crate::schema::products::dsl::*;
use crate::schema::categories::dsl::*;
use crate::database::insert_table::Category;

use serde::Serialize;

#[derive(Serialize)]
struct IndexContext {
    items: Vec<Product>,
    categoriess: Vec<Category>,
}

#[get("/")]
pub async fn list_of_item(pool: &State<DbPool>) -> Template {
    let mut pool1 = pool.get().unwrap();
    let result = tokio::task::spawn_blocking(move || {
        let items_result = products.load::<Product>(&mut pool1);
        let categories_result = categories.load::<Category>(&mut pool1);
        match (items_result, categories_result) {
            (Ok(items), Ok(categoriess)) => Ok(IndexContext { items, categoriess }),
            _ => Err("Failed to load data"),
        }
    })
    .await;

    match result {
        Ok(Ok(context)) => Template::render("index1", &context),
        _ => {
            let error_context = "Не вдалося завантажити продукти або категорії";
            Template::render("error", &error_context)
        }
    }
}


#[get("/picture/<file..>")] 
pub async fn picture(file: PathBuf) -> Option<NamedFile> { 
    NamedFile::open(Path::new("uploads/").join(file)).await.ok() 
}


#[get("/try_insert")]
pub fn insetable(pool: &State<DbPool>) {
    let mut a1 = pool.get().unwrap();
    let a = InsertProd { 
        name: "testing", 
        price: 10, 
        description: Some("testing"), 
        file_path: "photo_2024-04-30_21-12-06.jpg".to_string(), 
        category_id: None,
    };

    diesel::insert_into(products)
    .values(a)
    .execute(&mut a1);

}




