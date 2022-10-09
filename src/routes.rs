use crate::unsplash::UnsplashClient;
use rocket::serde::json::{json, Value};

#[get("/get_photos?<page_number>&<per_page>")]
pub async fn get_photos(page_number: Option<i32>, per_page: Option<i32>) -> Value {
    let page_number = page_number.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);

    let unsplash_client = UnsplashClient::new();
    let result = unsplash_client.get_photos(page_number, per_page).await;

    match result {
        Ok(r) => r,
        Err(err) => json!({ "Error": format!("{}", err) }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;
    use rocket::{Build, Rocket};

    fn rocket() -> Rocket<Build> {
        rocket::build().mount("/", routes![get_photos])
    }

    #[test]
    fn test_get_photo() {
        let client = Client::tracked(rocket()).expect("valid `Rocket`");
        let response = client.get("/get_photos").dispatch();
        assert_eq!(response.status(), Status::Ok);
    }
}
