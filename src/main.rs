#[macro_use] extern crate rocket;

// mod dice;
// mod hand;
// mod game;

// use dice::*;
// use hand::*;
// // use game::*;

use gg_rust::blog::client::{create_project, establish_connection};
use gg_rust::blog::models::NewProject;
use rocket::http::Header;
use rocket::{Request, Response, serde};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

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


#[derive(Serialize, Debug, FromForm, Deserialize)]
struct Project {
  id: i32,
  title: String,
  description: String, 
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/projects")]
/// Return  json string of all current projects from DB{
    fn get_projects() -> Json<Project> {
    
    // Just for testing
    let project: Project = Project {
        id: 1,
        title: "Gnu Girl.com".to_string(),
        description: "Personal website and portfolio".to_string(),
    };
    Json(project)
}

#[get("/projects/<id>")]
/// return a project specified by id
fn get_project_by_id(id: i32) -> Json<Project> {
    let project: Project = Project {
        id: id,
        title: "Gnu Girl.com".to_string(),
        description: "Personal website and portfolio".to_string(),
    };
    Json(project)
}

#[post("/projects", format = "json", data = "<project>")]
/// Create a new project & eventually return status code?
fn new_project(project:Json<NewProject>) {
    
    // temporary here - create tables

    let new_project = NewProject {id:project.id, title: project.title.clone(), description: project.description.clone()};
    let conn = & mut establish_connection();
    create_project(conn, new_project);
}
// #[get("/roll")]
// fn roll() -> String {
    
//     // println!("{}", mock_data::hello_world());
//     serde_json::to_string(&Die::roll()).unwrap()
// }

//TODO: Refactor code to create new post
// #[post("/newPost", data="<post>")]
// fn new_post(post:Form<NewPost>) -> () {
//     let draft = if post.draft { "draft" } else { "post" };
//     println!("Creating new {:?} with title: {:?}", draft,post.title);
//     let conn = & mut establish_connection();

//     create_post(conn, post.title.as_str(), post.body.as_str(), post.author.as_str(), post.draft);
// }


// #[get("/newHand")]
// fn new_hand() -> String {
//     serde_json::to_string(&Hand::new()).unwrap()
// }

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![get_projects])
        .mount("/", routes![get_project_by_id])
        .mount("/", routes![new_project])
        // .mount("/", routes![roll])
        // .mount("/", routes![new_hand])
        // .mount("/blog", routes![new_post])
        .attach(CORS)
}