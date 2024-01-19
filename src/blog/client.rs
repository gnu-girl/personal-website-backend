use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use crate::{blog::models::{NewPost, Post, PostQuery, NewProject}, schema::posts, schema::projects};
use crate::blog::queries::*;

use super::models::Project;

/// Create the needed tables in the database
pub fn create_tables(conn: &mut PgConnection) {
    diesel::sql_query("CREATE TABLE projects(id SERIAL PRIMARY KEY, title VARCHAR, description VARCHAR)").execute(conn).expect("ERRO CREATING PROJECTS TABLE");
}

/// Drop all tables in the database
pub fn drop_tables(conn: &mut PgConnection) {
    diesel::sql_query("DROP TABLE IF EXISTS projects").execute(conn).expect("ERROR DROPPING TABLE");
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_post(conn: &mut PgConnection, title: &str, body: &str, author: &str, published: bool) -> Post {

    let new_post = NewPost { title, body, author, published };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

/// Crate new entry in proejcts table
pub fn create_project(conn: &mut PgConnection, new_project: NewProject) -> Project {
    diesel::insert_into(projects::table)
        .values(&new_project)
        .get_result(conn)
        .expect("Error saving project")
}

/// Read all entries in projects table
pub fn read_projects(conn: &mut PgConnection) -> Vec<Project> {
    use crate::schema::projects::dsl::*;

    projects.select(Project::as_select())
    .load(conn)
    .expect("Error loading projects")

}

/// Return the project matching the given id
pub fn read_project_by_id(conn: &mut PgConnection, new_id:i32) -> Project {
    use crate::schema::projects::dsl::*;

    projects.select(Project::as_select())
    .filter(id.eq(new_id))
    .first(conn)
    .expect("Error finding project with given id")
}

/// Update the project entry matching the given id and return the updated record
pub fn update_project_by_id(conn: &mut PgConnection, new_id:i32, new_project: NewProject) -> Project {
    use crate::schema::projects::dsl::*;

    diesel::update(projects)
        .filter(id.eq(new_id))
        .set((
            title.eq(new_project.title),
            description.eq(new_project.description)
        ))
        .get_result(conn)
        .expect("ERROR UPDATING PROJECT")
}
pub fn publish_draft_post(conn: &mut PgConnection, query_id: i32) -> Post {
    use crate::schema::posts::dsl::*;

    diesel::update(posts.find(query_id))
        .set(published.eq(true))
        .get_result::<Post>(conn)
        .unwrap()
}

pub fn delete_post(conn: &mut PgConnection, query_id: i32) -> usize {
    use crate::schema::posts::dsl::*;
    
    diesel::delete(posts.filter(id.eq(query_id)))
        .execute(conn)
        .expect("Error deleting posts")
}

// Return published posts matching id
pub fn find_post(conn: &mut PgConnection, query_id: i32) -> Vec<Post> {
    use crate::schema::posts::dsl::*;
    
    //TODO: Actually use the queries here
    // let results = Post
    //     ::by_id(&id)
    //     .load::<Post>(conn)
    //     .expect(format!("Can't find post with given id {:?}", id).as_str());

    let results = posts
        .filter(published.eq(true))
        .filter(id.eq(query_id))
        .limit(1)
        .load::<Post>(conn)
        .expect("Error loading posts");
    results
}