use reqwest::Client;
use std::error::Error;

pub struct UnsplashClient {
    client: Client,
}

impl UnsplashClient {
    pub fn new() -> UnsplashClient {
        let client = reqwest::Client::new();
        UnsplashClient { client }
    }

    pub async fn get_page(&self) -> Result<String, Box<dyn Error>> {
        Ok(self
            .client
            .get("https://www.google.com/")
            .send()
            .await?
            .text()
            .await?)
    }
}
