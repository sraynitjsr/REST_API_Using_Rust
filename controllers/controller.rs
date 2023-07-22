use rocket::get;
use rocket::post;
use rocket::put;
use rocket::delete;
use rocket::response::content::Json;

struct User {
    id: u32,
    name: String,
    email: String,
}

struct UserDatabase {
    users: Vec<User>,
}

impl UserDatabase {
    fn new() -> Self {
        let users = vec![
            User {
                id: 1,
                name: "A".to_string(),
                email: "A007@gmail.com".to_string(),
            },
            User {
                id: 2,
                name: "B".to_string(),
                email: "B007@gmail.com".to_string(),
            },
        ];
        UserDatabase { users }
    }

    fn get_user_by_id(&self, user_id: u32) -> Option<&User> {
        self.users.iter().find(|user| user.id == user_id)
    }

    fn get_all_users(&self) -> &Vec<User> {
        &self.users
    }

    fn add_user(&mut self, user: User) {
        self.users.push(user);
    }

    fn update_user(&mut self, updated_user: User) -> bool {
        if let Some(index) = self.users.iter().position(|user| user.id == updated_user.id) {
            self.users[index] = updated_user;
            true
        } else {
            false
        }
    }

    fn delete_user(&mut self, user_id: u32) -> bool {
        if let Some(index) = self.users.iter().position(|user| user.id == user_id) {
            self.users.remove(index);
            true
        } else {
            false
        }
    }
}

impl UserDatabase {
    fn get_instance() -> &'static UserDatabase {
        static INSTANCE: UserDatabase = UserDatabase::new();
        &INSTANCE
    }
}

struct UserController;

impl UserController {
    #[get("/users")]
    pub fn get_all_users() -> Json<&'static Vec<User>> {
        Json(UserDatabase::get_instance().get_all_users())
    }

    #[get("/users/<id>")]
    pub fn get_user(id: u32) -> Option<Json<&'static User>> {
        UserDatabase::get_instance().get_user_by_id(id).map(Json)
    }

    #[post("/users", data = "<user>")]
    pub fn add_user(user: Json<User>) -> &'static str {
        UserDatabase::get_instance().add_user(user.into_inner());
        "User added successfully!"
    }

    #[put("/users", data = "<user>")]
    pub fn update_user(user: Json<User>) -> &'static str {
        if UserDatabase::get_instance().update_user(user.into_inner()) {
            "User updated successfully!"
        } else {
            "User not found."
        }
    }

    #[delete("/users/<id>")]
    pub fn delete_user(id: u32) -> &'static str {
        if UserDatabase::get_instance().delete_user(id) {
            "User deleted successfully!"
        } else {
            "User not found."
        }
    }
}
