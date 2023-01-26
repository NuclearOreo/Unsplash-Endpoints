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

        Ok(self
            .client
            .get(url)
            .header(AUTHORIZATION, auth)
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?)
    }
}

// In case I forgot, here's a blog for testing reqwest: https://write.as/balrogboogie/testing-reqwest-based-clients

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testing_client_auth() {
        let client = UnsplashClient::new();

        let auth = match env::var("unsplash_client_id") {
            Ok(v) => v,
            Err(_) => "".to_string(),
        };

        assert_eq!(client.auth, auth);
    }
}
