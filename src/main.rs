use std::error::Error;
use regex::Regex;
use reqwest::get;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() {
    scrape_url("https://www.amazon.com/").await.unwrap()
}

async fn scrape_url(url: &str) -> Result<(), Box<dyn Error>> {
    let response = get(url)
        .await?
        .text()
        .await?;
    
    if has_valid_url(&url) {
        let document = Html::parse_document(&response);
        let selector = Selector::parse("p").unwrap();
        for element in document.select(&selector) {
            let text: String = element.text().collect();
            println!("Found {}", text);
        }
    }
    
    Ok(())
}

fn has_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^(http|https)://(www\.)?[\w-]+(\.[\w-]+)+[/#?]?.*$").unwrap();
    re.is_match(url)
}