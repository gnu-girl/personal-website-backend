// use gg_rust::blog::client::{establish_connection, create_post};
use std::io::{stdin, Read};

/* Use user input to create a blog post */
fn main() {
//     let connection = &mut establish_connection();

//     let mut title = String::new();
//     let mut body = String::new();
//     let mut author = String::new();

//     println!("Who are you?");
//     stdin().read_line(&mut author).unwrap();
//     let author = author.trim_end(); // Remove the trailing newline

//     println!("What would you like your title to be?");
//     stdin().read_line(&mut title).unwrap();
//     let title = title.trim_end(); // Remove the trailing newline

//     println!(
//         "\nOk! Let's write {} (Press {} when finished)\n",
//         title, EOF
//     );
//     stdin().read_to_string(&mut body).unwrap();

//     let post = create_post(connection, title, &body, author, false);
//     println!("\nSaved draft {} with id {}", title, post.id);
}

#[cfg(not(windows))]
const EOF: &str = "CTRL+D";

#[cfg(windows)]
const EOF: &str = "CTRL+Z";