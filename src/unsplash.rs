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
    use mockito::mock;
    use serde_json::json;

    #[tokio::test]
    async fn test_get_photos() {
        let _m = mock(
            "GET",
            "https://api.unsplash.com/users/ussamaazam/photos?page=1&per_page=10",
        )
        .with_status(200)
        .with_body(r#"{"id":1,"name":"Test"}"#)
        .create();

        let client = reqwest::Client::new();
        let unsplash_client = UnsplashClient {
            client,
            auth: "".to_string(),
        };
        let result = unsplash_client.get_photos(1, 10).await;

        assert!(result.is_ok());

        // let value = result.unwrap();

        // println!("{:?}", value);
        // println!("Hello");

        // let json = json!({"id": 1, "name": "Test"});

        // assert_eq!(value, json);
    }
}
