extern crate reqwest;
use std::error::Error;

#[tokio::main]
async fn main() {
    scrape_url("https://www.amazon.com/").await.unwrap()
}

async fn scrape_url(url: &str) -> Result<(), Box<dyn Error>> {
    let url = reqwest::get(url)
        .await?
        .text()
        .await?;
    
    println!("{}", url);
    
    Ok(())
}