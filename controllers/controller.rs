use rocket::get;
use rocket::response::content::Json;

struct User {
    id: u32,
    name: String,
    email: String,
}

fn get_user_by_id(user_id: u32) -> Option<User> {
    let users = vec![
        User {
            id: 1,
            name: "A".to_string(),
            email: "A1@gmail.com".to_string(),
        },
        User {
            id: 2,
            name: "B".to_string(),
            email: "B2@gmail.com".to_string(),
        },
    ];

    users.into_iter().find(|user| user.id == user_id)
}

pub fn get_user(id: u32) -> Option<Json<User>> {
    if let Some(user) = get_user_by_id(id) {
        Some(Json(user))
    } else {
        None
    }
}
