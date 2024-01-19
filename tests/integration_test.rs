use gg_rust::blog::{client::*, models::NewProject};
use gg_rust::errors::*;
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
    assert_eq!(read_project_by_id(&mut conn, 1).unwrap().id, project.id);
    assert_eq!(read_project_by_id(&mut conn, 1).unwrap().title, project.title);
    assert_eq!(read_project_by_id(&mut conn, 1).unwrap().description, project.description);
    println!("SUCCESSFULLY COMPLETED STEP 6: Finding item with id=1");

    // Step 7: Update all fields of one specific item
    let mut updated_project_fields = NewProject {title: "New Title111".to_owned(), description: "New Description111".to_owned()};
    let mut updated_project_from_db = update_project_by_id(&mut conn, 1, updated_project_fields.clone());
    let project_w_id_2 = read_project_by_id(&mut conn, 2).unwrap();

    assert_eq!(updated_project_fields.title, updated_project_from_db.title);
    assert_eq!(updated_project_fields.description, updated_project_from_db.description);
    assert_ne!(updated_project_fields.title, project_w_id_2.title);
    assert_ne!(updated_project_fields.description, project_w_id_2.description);
    println!("SUCCESSFULLY COMPLETED STEP 7: Updating all fields of a single item");

    // Step 8: Update a single field of a given item
    updated_project_fields.title = "Updated Title: Round 2".to_owned();
    updated_project_from_db = update_project_by_id(&mut conn, 1, updated_project_fields.clone());
    let project_w_id_2 = read_project_by_id(&mut conn, 2).unwrap();

    // Checking updating just the title
    assert_eq!(updated_project_fields.title, updated_project_from_db.title);
    assert_eq!(updated_project_fields.description, updated_project_from_db.description);
    assert_ne!(updated_project_fields.title, project_w_id_2.title);
    assert_ne!(updated_project_fields.description, project_w_id_2.description);

    // Checking updating just the description
    updated_project_fields.description = "Updated description: Round 2".to_owned();
    updated_project_from_db = update_project_by_id(&mut conn, 1, updated_project_fields.clone());

    assert_eq!(updated_project_fields.title, updated_project_from_db.title);
    assert_eq!(updated_project_fields.description, updated_project_from_db.description);
    assert_ne!(updated_project_fields.title, project_w_id_2.title);
    assert_ne!(updated_project_fields.description, project_w_id_2.description);
    println!("SUCCESSFULLY COMPLETED STEP 8: Updating individual fields of a single item");

    // Step 9: Deleting an item by an id
    let deleted_project = delete_project_by_id(&mut conn, 1);
    assert_eq!(deleted_project.id,1);
    assert_eq!(deleted_project.title,updated_project_from_db.title);
    assert_eq!(deleted_project.description,updated_project_from_db.description);
    assert!(read_project_by_id(&mut conn, 1).is_err());
    // assert_eq!(read_project_by_id(&mut conn, 1), Err("Error finding project with given id"));

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