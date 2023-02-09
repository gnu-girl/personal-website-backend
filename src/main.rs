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
use rocket::form::Form;
use gg_rust::blog::client::*;

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
    draft: bool
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

//TODO: Refactor code to create new post
// #[post("/newPost", data="<post>")]
// fn new_post(post:Form<NewPost>) -> () {
//     let draft = if post.draft { "draft" } else { "post" };
//     println!("Creating new {:?} with title: {:?}", draft,post.title);
//     let conn = & mut establish_connection();

//     create_post(conn, post.title.as_str(), post.body.as_str(), post.author.as_str(), post.draft);
// }


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
        // .mount("/blog", routes![new_post])
        .attach(CORS)
}