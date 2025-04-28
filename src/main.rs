use std::error::Error;
use dotenv::dotenv;
use log::info;
use log::warn;
use regex::Regex;
use roxmltree::Document;

#[tokio::main]
async fn load_sitemap(sitemap_location: &str) -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get(sitemap_location).await?;
    let body = resp.text().await?;
    Ok(body)
}

#[tokio::main]
async fn load_robots_txt(website_url: &str) ->  Result<String, Box<dyn Error>> {
    let url: String = [website_url, "/robots.txt"].join("");

    info!("Looking for robots.txt at {}", url);

    let resp = reqwest::get(url).await?;
    let body = resp.text().await?;

    info!("robots.txt content {}", body.replace("\n", ""));

    Ok(body)
}

fn get_crawl_delay_from_robots_txt(website_url: &str ) -> Option<i8>  {
    match load_robots_txt(website_url) {
        Ok(body) => {
            match Regex::new(r"Crawl-delay:\s+(\d+)$") {
                Ok(re) => {
                    if let Some(matches) = re.captures(&body) {
                        println!("Matches {:?}", matches);
                    }
                },
                Err(e) => {
                    warn!("Crawl-delay not found in robots.txt {}", e);
                }
            }        
        },
        Err(e) => {
            warn!("robots.txt not found {}", e);
        }
    }
    return None;
}

fn parse_sitemap(sitemap: &str) -> Result<Vec<String>, roxmltree::Error> {
    let mut urls = Vec::new();
    match Document::parse(sitemap) {
        Ok(doc) => {
            for node in doc.descendants() {
                if node.tag_name().name() == "loc" {
                    if let Some(text) = node.text() {
                        urls.push(text.to_string());
                    }
                }
            }
            Ok(urls)
        },
        Err(e) => {
            warn!("Sitemap not found or error parsing sitemap: {}", e);
            Err(e)
        }
    }
}

fn main() {
    dotenv().ok();
    env_logger::init();
    let website_url: String = std::env::var("WEBSITE_URL").expect("WEBSITE_URL environment variable missing");
    let sitemap_location: String = std::env::var("SITEMAP_LOCATION").expect("SITEMAP_LOCATION environment variable missing");
    let mut crawl_delay: i8 = std::env::var("CRAWL_DELAY").expect("CRAWL_DELAY environment variable missing").parse().unwrap();

    info!("Website url environment variable loaded {}", website_url);
    info!("Sitemap location environment variable loaded {}", sitemap_location);
    info!("Crawl Delay environment variable loaded with value {} seconds", crawl_delay);

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
    match result {
        Ok(body) => {
            info!("Sitemap body: {}", body.replace("\n", ""));
            let urls = parse_sitemap(&body);

            info!("Sitemap urls: {:?}", urls);

        },
        Err(_e) => {
            panic!("Could not load sitemap from location {}", sitemap_location)
        }
    }

}