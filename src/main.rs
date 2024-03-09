use std::error::Error;
use dotenv::dotenv;
use log::info;
use env_logger;


#[tokio::main]
async fn load_sitemap(sitemap_location: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(sitemap_location).await?;
    let body = resp.text().await?;

    Ok(body)
}


fn main() {
    dotenv().ok();
    env_logger::init();
    let website_url: String = std::env::var("WEBSITE_URL").expect("WEBSITE_URL environment variable missing");
    let sitemap_location: String = std::env::var("SITEMAP_LOCATION").expect("SITEMAP_LOCATION environment variable missing");

    info!("Website url environment variable loaded {}", website_url);
    info!("Sitemap location environment variable loaded {}", sitemap_location);

    let result = load_sitemap(&sitemap_location);
    info!("{:?}", result)
}