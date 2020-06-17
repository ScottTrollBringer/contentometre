use crate::rocket;
use rocket::local::Client;
use rocket::http::{Status, ContentType};

#[test]
fn get() {
    let client = Client::new(rocket()).unwrap();
    let mut res = client.get("/").header(ContentType::JSON).dispatch();
    assert_eq!(res.status(), Status::Ok);
    let body = res.body().unwrap().into_string().unwrap();
    assert!(body.contains("Hello, World!"));
}