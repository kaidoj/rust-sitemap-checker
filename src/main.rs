use std::error::Error;
use dotenv::dotenv;
use log::info;
use log::warn;
use regex::Regex;


#[tokio::main]
async fn load_sitemap(sitemap_location: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(sitemap_location).await?;
    let body = resp.text().await?;

    Ok(body)
}

#[tokio::main]
async fn load_robots_txt(website_url: &str) ->  String {
    let url: String = [website_url, "/robots.txt"].join("");

    info!("Looking for robots.txt at {}", url);

    let resp = reqwest::get(url).await.unwrap();
    let body = resp.text().await.unwrap();

    info!("robots.txt content {}", body.replace("\n", ""));

    return body
}

fn get_crawl_delay_from_robots_txt(website_url: &str ) -> Option<i8>  {
    let body = load_robots_txt(website_url);
    let re = Regex::new("Crawl-delay:s+(d+)$").unwrap();
    let Some(matches) = re.captures(&body) else {
        return None;
    };

    println!("Matches {:?}", matches);

    return None;
}


fn main() {
    dotenv().ok();
    env_logger::init();
    let website_url: String = std::env::var("WEBSITE_URL").expect("WEBSITE_URL environment variable missing");
    let sitemap_location: String = std::env::var("SITEMAP_LOCATION").expect("SITEMAP_LOCATION environment variable missing");
    let mut crawl_delay: i8 = std::env::var("CRAWL_DELAY").expect("CRAWL_DELAY environment variable missing").parse().unwrap();

    info!("Website url environment variable loaded {}", website_url);
    info!("Sitemap location environment variable loaded {}", sitemap_location);

    let robots_txt_crawl_delay = get_crawl_delay_from_robots_txt(&website_url);
    match robots_txt_crawl_delay {
        Some(delay) => {
            crawl_delay = delay;
            info!("Found Crawl-delay from robots.txt {} seconds", crawl_delay);
        },
        None => {
            warn!("robots.txt did not contain Crawl-delay. Using default {} seconds", crawl_delay);
        }
    }
    
    let result = load_sitemap(&sitemap_location);
    info!("{:?}", result)
}