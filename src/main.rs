#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Amazing I got this!!"
}

#[get("/hello")]
async fn hello() -> String {
    let client = reqwest::Client::new();

    let response = client
        .get("https://httpbin.org/ip")
        .send()
        .await
        .unwrap()
        .text()
        .await;

    let resp = match response {
        Ok(response) => response,
        Err(err) => err.to_string(),
    };

    resp
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
