#[macro_use] extern crate rocket;

mod dice;
mod hand;
mod game;

use dice::*;
use hand::*;
use game::*;


#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/roll")]
fn roll() -> String {
    serde_json::to_string(&Die::roll()).unwrap()
}

#[get("/newHand")]
fn new_hand() -> String {
    serde_json::to_string(&Hand::new()).unwrap()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![roll])
        .mount("/", routes![new_hand])
}