#[macro_use] extern crate rocket;

mod dice;
mod hand;
mod game;

use dice::*;
use hand::*;
// use game::*;

use rocket::http::Header;
use rocket::{Request, Response, serde};
use rocket::fairing::{Fairing, Info, Kind};
use rust_zee::blog::mock_data;
use rust_zee::blog::structures::Post;
use rocket::serde::{Deserialize};
use rocket::form::Form;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(FromForm)]
struct NewPost {
    title: String,
    author: String,
    body: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/roll")]
fn roll() -> String {
    
    // println!("{}", mock_data::hello_world());
    serde_json::to_string(&Die::roll()).unwrap()
}

#[post("/newPost", data="<post>")]
fn new_post(post:Form<NewPost>) -> () {
    println!("Woah!! NEW post!!! {:?}", post.title);
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
        .mount("/blog", routes![new_post])
        .attach(CORS)
}