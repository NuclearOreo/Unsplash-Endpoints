mod routes;
mod unsplash;

use routes::get_photos;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Service is running 🚀"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/unsplash/", routes![get_photos])
}

#[cfg(test)]
mod tests {
    use super::*;
    use rocket::http::Status;
    use rocket::local::blocking::Client;

    #[test]
    fn test_index() {
        let res = index();
        assert_eq!("Service is running 🚀", res)
    }

    #[test]
    fn rocket_index() {
        let client = Client::tracked(rocket()).expect("valid `Rocket`");
        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.into_string().unwrap(), "Service is running 🚀");
    }
}
