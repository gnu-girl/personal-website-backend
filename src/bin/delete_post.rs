use diesel::prelude::*;
use gg_rust::blog::client::establish_connection;
use std::env::args;

/* Delete post with matching title */
fn main() {
    use gg_rust::schema::posts::dsl::*;

    let target = args().nth(1).expect("Expected a target to match against");
    let pattern = format!("%{}%", target);

    let connection = &mut establish_connection();
    let num_deleted = diesel::delete(posts.filter(title.like(pattern)))
        .execute(connection)
        .expect("Error deleting posts");

    println!("Deleted {} posts", num_deleted);
}