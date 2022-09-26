use crate::unsplash::UnsplashClient;

#[get("/hello")]
pub async fn hello() -> String {
    let unsplash_client = UnsplashClient::new();
    let result = unsplash_client.get_page().await;

    match result {
        Ok(r) => r,
        Err(err) => format!("Server Side Error: {}", err),
    }
}
