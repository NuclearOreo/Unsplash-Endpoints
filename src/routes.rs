use crate::unsplash::UnsplashClient;

#[get("/get_photos?<page_number>&<per_page>")]
pub async fn get_photos(page_number: Option<i32>, per_page: Option<i32>) -> String {
    let page_number = match page_number {
        Some(v) => v,
        None => 1,
    };

    let per_page = match per_page {
        Some(v) => v,
        None => 10,
    };

    let unsplash_client = UnsplashClient::new();
    let result = unsplash_client.get_photos(page_number, per_page).await;

    match result {
        Ok(r) => r,
        Err(err) => format!("Server Side Error: {}", err),
    }
}
