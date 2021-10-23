#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Value};

// utils
fn number_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }

    for a in 2..n {
        if n % a == 0 {
            return false;
        }
    }

    true
}

// Routes and controllers
#[get("/")]
async fn home() -> Value {
    json!({
        "status": "ok",
        "message": "Hola Mundo!"
    })
}

#[get("/api/<number>")]
async fn index(number: u64) -> Value {
    json!({
        "message": "Hola mundo",
        "es primo": number_prime(number)
    })
}

// Main Launch
#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index, home]).launch().await;
}
