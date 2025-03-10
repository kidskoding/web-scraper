pub mod server;

extern crate reqwest;
use regex::Regex;
use scraper::{Html, Selector};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tokio::spawn(async {
        match server::serve().await {
            Ok(url) => url,
            Err(e) => {
                eprintln!("Server error: {}", e);
            }
        }
    });

    tokio::time::sleep(tokio::time::Duration::from_millis(100))
        .await;

    let url = "http://localhost:3000";
    scrape_url(url).await?;

    Ok(())
}

async fn scrape_url(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url)
        .await?
        .text()
        .await?;

    if has_valid_url(&url) {
        let document = Html::parse_document(&response);
        let selector = Selector::parse("p").unwrap();
        for element in document.select(&selector) {
            let text: String = element.text().collect();
            println!("{url} contains the following text: {text}");
        }
    }

    Ok(())
}

fn has_valid_url(url: &str) -> bool {
    let re = Regex::new(r"^(http|https)://(www\.)?[\w-]+(\.[\w-]+)+[/#?]?.*$").unwrap();
    let localhost_re = Regex::new(r"localhost:").unwrap();
    re.is_match(url) || localhost_re.is_match(url)
}
