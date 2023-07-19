#[macro_use]
extern crate rocket;

use rocket::serde::json::{Json, Value};
use rocket::State;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateUser {
    name: Option<String>,
    email: Option<String>,
}

struct AppState {
    users: Vec<User>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[get("/users")]
fn get_users(state: &State<AppState>) -> Json<Vec<User>> {
    Json(state.users.clone())
}

#[put("/users/<id>", data = "<update>")]
fn update_user(id: u32, update: Json<UpdateUser>, state: &State<AppState>) -> Option<Json<User>> {
    let mut users = state.users.clone();

    for user in &mut users {
        if user.id == id {
            if let Some(name) = &update.name {
                user.name = name.clone();
            }
            if let Some(email) = &update.email {
                user.email = email.clone();
            }
            return Some(Json(user.clone()));
        }
    }

    None
}

#[launch]
fn rocket() -> _ {
    let users = vec![
        User {
            id: 1,
            name: String::from("A"),
            email: String::from("A$1@example.com"),
        },
        User {
            id: 2,
            name: String::from("B"),
            email: String::from("B2%@example.com"),
        },
    ];

    rocket::build()
        .manage(AppState { users })
        .mount("/", routes![index, get_users, update_user])
}
