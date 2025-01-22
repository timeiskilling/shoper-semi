use std::path::{Path, PathBuf};

use diesel::RunQueryDsl;
use rocket::fs::NamedFile;
use rocket::State;
use rocket_dyn_templates::Template;
use crate::database::insert_table::Product;
use crate::database::DbPool;
use crate::schema::products::dsl::*;
use rocket::http::ContentType;



#[get("/")]
pub async fn list_of_item(pool: &State<DbPool>) -> Template {
    let mut conn = pool.get().unwrap();
    let result = tokio::task::spawn_blocking(move || {
        products
            .load::<Product>(&mut conn)
            .unwrap()
    })
    .await;

    if let Ok(vec) = result {
        let mut context = std::collections::HashMap::new();
        context.insert("items", vec);

        Template::render("index1", &context)
    } else {
        let mut context = std::collections::HashMap::new();
        context.insert("error", "Failed to load product details");

        Template::render("error", &context)
    }
}

#[get("/picture/<file..>")] 
pub async fn picture(file: PathBuf) -> Option<NamedFile> { 
    NamedFile::open(Path::new("uploads/").join(file)).await.ok() 
}



fn get_content_type(path: &Path) -> ContentType {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("css") => ContentType::CSS,
        Some("js") => ContentType::JavaScript,
        Some("jpg") => ContentType::JPEG,
        Some("jpeg") => ContentType::JPEG,
        Some("png") => ContentType::PNG,
        Some("gif") => ContentType::GIF,
        Some("html") => ContentType::HTML,
        _ => ContentType::Plain,
    }
}


#[get("/<file..>")]
pub async fn static_files(file: PathBuf) -> Option<(ContentType, NamedFile)> {
    let path = Path::new("templaiting/").join(file);
    NamedFile::open(&path).await.ok().map(|file| (get_content_type(&path), file))
}