use reqwest::Client;
use std::error::Error;

#[get("/hello")]
pub async fn hello() -> String {
    let unsplash_client = UnsplashClient::new();
    let result = unsplash_client.get_page().await;

    match result {
        Ok(r) => r,
        Err(err) => format!("Server Side Error: {}", err),
    }
}

struct UnsplashClient {
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
