extern crate rocket;

mod controller;

fn main() {
    rocket::ignite().mount("/", routes![controller::hello]).launch();
}
