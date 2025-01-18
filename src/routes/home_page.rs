use std::path::{Path, PathBuf};

use diesel::RunQueryDsl;
use rocket::fs::NamedFile;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;


#[get("/")]
pub async fn list_of_item(pool: &State<DbPool>) -> Template {
    let result = products
        .load::<Product>(&mut pool.get().expect("Failed to get connection"))
        .expect("Error loading items");

    let mut context = std::collections::HashMap::new();
    context.insert("items", result); 

    Template::render("index1", &context)
}

#[get("/picture/<file..>")] 
pub async fn picture(file: PathBuf) -> Option<NamedFile> { 
    NamedFile::open(Path::new("uploads/").join(file)).await.ok() 
}

