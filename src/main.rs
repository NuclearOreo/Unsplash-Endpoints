#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Amazing I got this!!"
}

#[get("/hello")]
fn hello() -> &'static str {
    "Amazing I got this!!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![hello])
}
