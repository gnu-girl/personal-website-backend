use gg_rust::blog::{client::*, models::NewProject};
use rand::Rng;
use random_word::Lang;
use rocket::{form::validate::Len, serde::json::Json, tokio::fs::read};

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
    let project = create_project(&mut conn, gen_project());
    println!("SUCCESSFULLY COMPLETED STEP 3: Inserting 1 item with id={}", project.id);

    // Step 4: Insert n items into projects table
    let mut rng = rand::thread_rng();
    let n = rng.gen_range(1..100);

    for _ in 0..n {
        create_project(&mut conn, gen_project());
    }
    println!("SUCCESSFULLY COMPLETED STEP 4: Inserting n={} items", n);

    // Step 5: Reading all entries
    let read_projects = read_projects(&mut conn);
    assert_eq!(read_projects.len(),n+1);
    println!("SUCCESSFULLY COMPLETED STEP 5: Reading all ({}) items from table", read_projects.len());

    // Step 6: Reading the first entry by ID lookup
    assert_eq!(read_project_by_id(&mut conn, 1).id, project.id);


}

fn gen_project() -> NewProject{
    let mut rng = rand::thread_rng();
    let rand_title_length: u32 = rng.gen_range(1..10);
    let rand_description_length: u32 = rng.gen_range(1..100);

    let mut title = String::new();
    let mut description= String::new();

    for _ in 0..rand_title_length {
        title.push_str(random_word::gen(Lang::En));
        title.push_str(" ");
    }

    for _ in 0..rand_description_length {
        description.push_str(random_word::gen(Lang::En));
        description.push_str(" ");
    }

    NewProject {
        title: title,
        description: description
    }
}