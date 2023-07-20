#[macro_use]
extern crate rocket;

use rocket::http::Status;
use rocket::serde::json::{Json, Value};
use rocket::State;

struct User {
    id: u32,
    name: String,
    email: String,
}

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

#[delete("/users/<email>")]
fn delete_user(email: &str, state: &State<AppState>) -> Status {
    let mut users = state.users.clone();

    let user_idx = users.iter().position(|user| user.email == email);

    if let Some(idx) = user_idx {
        users.remove(idx);
        Status::NoContent
    } else {
        Status::NotFound
    }
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<User>, state: &State<AppState>) -> Status {
    let mut users = state.users.clone();

    // Check if the email already exists
    if users.iter().any(|u| u.email == user.email) {
        return Status::Conflict;
    }

    // Find the highest user ID to generate a new unique ID
    let max_id = users.iter().map(|u| u.id).max().unwrap_or(0);
    let new_user_id = max_id + 1;

    let new_user = User {
        id: new_user_id,
        name: user.name.clone(),
        email: user.email.clone(),
    };

    users.push(new_user);
    state.users = users;
    Status::Created
}

#[launch]
fn rocket() -> _ {
    let users = vec![
        User {
            id: 1,
            name: String::from("Subhradeep Ray"),
            email: String::from("Ray001@example.com"),
        },
        User {
            id: 2,
            name: String::from("Tufan Ray"),
            email: String::from("Tufan001@example.com"),
        },
    ];

    rocket::build()
        .manage(AppState { users })
        .mount("/", routes![index, get_users, update_user, delete_user, create_user])
}
