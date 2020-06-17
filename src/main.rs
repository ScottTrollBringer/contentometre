#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate serde_derive;

#[cfg(test)] mod tests;

use rocket_contrib::json::{Json};

#[derive(Serialize, Deserialize)]
struct Hello {
    message: String
}

#[get("/", format = "json")]
fn index() -> Json<Hello> {
    Json(Hello {
        message: "Hello, World!".to_string()
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount("/", routes![index])
}

fn main() {
    rocket().launch();
}