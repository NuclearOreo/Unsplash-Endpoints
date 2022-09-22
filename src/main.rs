#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Amazing I got this!!"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
