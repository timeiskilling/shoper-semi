use rocket::State;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket_sync_db_pools::{database, diesel};
use diesel::prelude::*;
use tokio::task::spawn_blocking;
use uuid::Uuid;
use schema::products;
use crate::schema;
use crate::database::DbPool;


#[derive(Insertable, Debug, Clone)]
#[diesel(table_name = products)]
pub struct InsertProds {
    pub name: String,
    pub price: i32,
    pub description: Option<String>,
    pub file_path: String,
    pub category_id: Option<i32>,
}

#[derive(FromForm)]
pub struct ProductForm<'r> {
    pub name: &'r str,
    pub price: i32,
    pub description: Option<&'r str>,
    pub category_id: Option<i32>,
    pub file: TempFile<'r>,
}

#[post("/add_product", data = "<form>")]
pub async fn add_product(mut form: Form<ProductForm<'_>>, pool: &State<DbPool>) -> Result<&'static str, String> {
    let mut pool1 = pool.get().unwrap();

    if !form.file.content_type().map_or(false, |ct| ct.is_png() || ct.is_jpeg()) {
        return Err("Неприпустимий тип файлу".into());
    }

    let extension = match form.file.content_type() {
        Some(ct) if ct.is_png() => "png",
        Some(ct) if ct.is_jpeg() => "jpg",
        _ => return Err("Невідомий формат файлу".into()),
    };
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let filepath = format!("uploads/{}", filename);

    save_file(&mut form.file, &filepath).await?;

    let new_product = InsertProds {
        name: form.name.to_string(),
        price: form.price,
        description: form.description.map(|s| s.to_string()),
        file_path: filepath.clone(),
        category_id: form.category_id,
    };

    let result = spawn_blocking(move || {
        diesel::insert_into(products::table)
            .values(&new_product)
            .execute(&mut pool1)
    }).await.map_err(|e| e.to_string())?;

    Ok("Продукт успішно додано!")
}

async fn save_file(file: &mut TempFile<'_>, filepath: &str) -> Result<(), String> {
    use tokio::fs;
    use std::path::Path;

    if let Some(parent) = Path::new(&filepath).parent() {
        fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
    }

    
    file.copy_to(&filepath).await.map_err(|e| e.to_string())?;

    Ok(())
}