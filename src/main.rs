use rocket::serde::json::{json, Value};
#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> Value {
    json!({
        "message": "Hola mundo",
        "value": 2 + 2
    })
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/", routes![index])
        .launch()
        .await;
}
