extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, World!"
}

fn main() {
    rocket::ignite().mount("/", routes![hello]).launch();
}
