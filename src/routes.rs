use crate::unsplash::UnsplashClient;

#[get("/get_photos")]
pub async fn get_photos() -> String {
    let unsplash_client = UnsplashClient::new();
    let result = unsplash_client.get_page().await;

    match result {
        Ok(r) => r,
        Err(err) => format!("Server Side Error: {}", err),
    }
}
