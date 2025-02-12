use crate::database::sorting::DbConn;
use reqwest::Client;


pub async fn scrap_product(db : &DbConn )  {
    let client = Client::new();

    let response = client.get("https://scrapeme.live/shop/")
        .send().await.unwrap();

    let html_content = response.text().await.unwrap();

    let document = scraper::Html::parse_document(&html_content);
}