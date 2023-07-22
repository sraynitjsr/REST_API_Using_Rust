extern crate rocket;

mod controller;

fn main() {
    rocket::ignite().mount("/", routes![
        controller::hello,
        controller::UserController::get_all_users,
        controller::UserController::get_user,
        controller::UserController::add_user,
        controller::UserController::update_user,
        controller::UserController::delete_user,
    ]).launch();
}
