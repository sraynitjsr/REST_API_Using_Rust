#[macro_use]
extern crate rocket;

use rocket::serde::json::Json;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/users")]
fn get_users() -> Json<Vec<User>> {
    let users = vec![
        User {
            id: 1,
            name: String::from("A"),
            email: String::from("A1#@example.com"),
        },
        User {
            id: 2,
            name: String::from("B"),
            email: String::from("B1^@example.com"),
        },
    ];

    Json(users)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, get_users])
}
