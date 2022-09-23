#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Amazing I got this!!"
}

#[get("/hello")]
async fn hello() -> String {
    let client = reqwest::Client::new();
    let response = client.get("https://www.google.com/").send().await;

    match response {
        Ok(res) => {
            let c = res.text().await;
            match c {
                Ok(t) => t,
                Err(_) => "Can't be parsed".to_string(),
            }
        }
        Err(_) => "Error from server".to_string(),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, hello])
}
