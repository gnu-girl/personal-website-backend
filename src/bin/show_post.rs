use rust_zee::models::*;
use diesel::prelude::*;
use rust_zee::establish_connection;
// use rust_zee::diesel_demo::*;

fn main() {
    use rust_zee::schema::posts::dsl::*;

    let connection = &mut establish_connection();
    let results = posts
        .filter(published.eq(true))
        .limit(5)
        .load::<Post>(connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for post in results {
        println!("{}", post.title);
        println!("-----------\n");
        println!("{}", post.body);
    }
}