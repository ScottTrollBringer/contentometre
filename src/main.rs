#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::json::{Json, JsonValue};

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

fn main() {
    rocket::ignite().mount("/", routes![index]).launch();
}
