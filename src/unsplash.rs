use reqwest::header::AUTHORIZATION;
use reqwest::Client;
use serde_json::Value;
use std::env;
use std::error::Error;

#[derive(Debug)]
pub struct UnsplashClient {
    client: Client,
    auth: String,
}

impl UnsplashClient {
    pub fn new() -> UnsplashClient {
        let client = reqwest::Client::new();
        let auth = match env::var("unsplash_client_id") {
            Ok(v) => v,
            Err(_) => "".to_string(),
        };
        UnsplashClient { client, auth }
    }

    pub async fn get_photos(
        &self,
        page_number: i32,
        per_page: i32,
    ) -> Result<Value, Box<dyn Error>> {
        let auth = self.auth.clone();
        let url = format!(
            "https://api.unsplash.com/users/ussamaazam/photos?page={}&per_page={}",
            page_number, per_page
        );

        let value = self
            .client
            .get(url)
            .header(AUTHORIZATION, auth)
            .send()
            .await?
            .json()
            .await?;

        Ok(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_photos() {
        let client = reqwest::Client::new();
        let unsplash_client = UnsplashClient {
            client,
            auth: "".to_string(),
        };
        let result = unsplash_client.get_photos(1, 10).await;

        // Assert if the call was successful
        assert!(result.is_ok());

        let response = result.unwrap();

        // Assert if response is an object
        assert!(response.is_object());
    }
}
