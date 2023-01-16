use rust_zee::blog::models::*;
use diesel::prelude::*;
use rust_zee::blog::client::{establish_connection,find_post};
use std::env::args;

/* Retrieve matching posts with given id */

fn main() {
    use rust_zee::schema::posts::dsl::*;

    let query_id = args()
        .nth(1)
        .expect("publish_post requires a post id")
        .parse::<i32>()
        .expect("Invalid ID");
    let connection = &mut establish_connection();
    let results = find_post(connection,query_id);
    // let results = posts
    //     .filter(published.eq(true))
    //     .limit(5)
    //     .load::<Post>(connection)
    //     .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}