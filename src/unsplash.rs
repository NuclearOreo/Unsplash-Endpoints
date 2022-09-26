use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use std::error::Error;

#[derive(Debug)]
pub struct UnsplashClient {
    client: Client,
    auth: String,
}

impl UnsplashClient {
    pub fn new() -> UnsplashClient {
        let client = reqwest::Client::new();
        let auth = "".to_string();
        UnsplashClient { client, auth }
    }

    pub async fn get_page(&self) -> Result<String, Box<dyn Error>> {
        let auth = self.auth.clone();

        Ok(self
            .client
            .get("https://api.unsplash.com/users/ussamaazam/photos")
            .header(AUTHORIZATION, auth)
            .send()
            .await?
            .text()
            .await?)
    }
}
