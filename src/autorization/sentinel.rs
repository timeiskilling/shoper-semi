use rocket::State;
use rocket::form::Form;
use rocket::fs::TempFile;
use rocket_sync_db_pools::{database, diesel};
use diesel::prelude::*;
use tokio::task::spawn_blocking;
use uuid::Uuid;
use schema::products;
use crate::database::insert_table::NewProductImage;
use crate::schema::{self, product_images};
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
    pub main_image: TempFile<'r>, 
    #[field(name = "images")]
    pub images: Vec<TempFile<'r>>, 
}

#[post("/add_product", data = "<form>")]
pub async fn add_product<'r>(mut form: Form<ProductForm<'r>>, pool: &State<DbPool>) -> Result<&'static str, String> {
    let mut conn = pool.get().unwrap();

    let main_image_extension = match form.main_image.content_type() {
        Some(ct) if ct.is_png() => "png",
        Some(ct) if ct.is_jpeg() => "jpg",
        _ => return Err("Невідомий формат головного зображення".into()),
    };

    let main_filename = format!("{}.{}", Uuid::new_v4(), main_image_extension);
    let main_filepath = format!("uploads/{}", main_filename);

    save_file(&mut form.main_image, &main_filepath).await?;

    let mut image_filenames = Vec::new();

    for image in form.images.iter_mut() {
        let image_extension = match image.content_type() {
            Some(ct) if ct.is_png() => "png",
            Some(ct) if ct.is_jpeg() => "jpg",
            _ => return Err("Невідомий формат додаткового зображення".into()),
        };

        let filename = format!("{}.{}", Uuid::new_v4(), image_extension);
        let filepath = format!("uploads/{}", filename);

        save_file(image, &filepath).await?;

        image_filenames.push(filename);
    }

    
    let new_product = InsertProds {
        name: form.name.to_string(),
        price: form.price,
        description: form.description.map(|s| s.to_string()),
        file_path: main_filename.clone(),
        category_id: form.category_id,
    };

    
    let result = spawn_blocking(move || {
        conn.transaction::<_, diesel::result::Error, _>(|conn| {

            diesel::insert_into(products::table)
                .values(&new_product)
                .execute(conn)?;

            let product_id = products::table
                .order(products::id.desc())
                .select(products::id)
                .first::<i32>(conn)?;


            let new_images: Vec<NewProductImage> = image_filenames.into_iter().map(|filename| {
                NewProductImage {
                    product_id,
                    file_path: filename,
                }
            }).collect();

            diesel::insert_into(product_images::table)
                .values(&new_images)
                .execute(conn)?;

            Ok(())
        })
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