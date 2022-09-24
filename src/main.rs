mod routes;

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
        .mount("/unsplash/", routes![routes::hello])
}
