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

    #[test]
    fn test_index() {
        let res = index();
        assert_eq!("Service is running 🚀", res)
    }
}
