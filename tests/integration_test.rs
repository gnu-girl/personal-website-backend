use gg_rust::blog::{client::*, models::NewProject};
use rand::Rng;
use random_word::Lang;
use rocket::serde::json::Json;

#[test]
fn system_test() {
    // Step 0: Create Connection to DB
    let mut conn = establish_connection();
    println!("SUCCESSFULLY COMPLETED STEP 0: Establishing Connection to DB");

    // Step 1: Drop tables (want to start from scratch)
    drop_tables(&mut conn);
    println!("SUCCESSFULLY COMPLETED STEP 1: Dropping current tables");

    // Step 2: Recreate tables with the correct tables
    create_tables(&mut conn);
    println!("SUCCESSFULLY COMPLETED STEP 2: Creating tables");

    // Step 3: Insert 1 item into projects table
    // let project = NewProject {id:7, title: "CATS!".to_owned(), description: "THIS is a description!".to_owned()};
    create_project(&mut conn, gen_project());
    println!("SUCCESSFULLY COMPLETED STEP 3: Inserting 1 item");

    // Step 4: Insert X items into projects table
    let mut rng = rand::thread_rng();

    for i in 0..rng.gen_range(1..100) {
        create_project(&mut conn, gen_project());
    }
}

fn gen_project() -> NewProject{
    let mut rng = rand::thread_rng();
    let rand_title_length: u32 = rng.gen_range(0..10);
    let rand_description_length: u32 = rng.gen_range(0..100);

    let mut title = String::new();
    let mut description= String::new();

    for i in 0..rand_title_length {
        title.push_str(random_word::gen(Lang::En));
        title.push_str(" ");
    }

    for i in 0..rand_description_length {
        description.push_str(random_word::gen(Lang::En));
        description.push_str(" ");
    }

    let p = NewProject {
        id: rng.gen_range(0..50),
        title: title,
        description: description
    };

    println!("Created new project with ID: {:?}", p.id);
    p
}