use chrono::prelude::*;
use snafu::prelude::*;
use image::*;
use uuid::Uuid;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("Unable to find pust with given id: {}", uuid))]
    PostRetrievalError { uuid: Uuid },
}

type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Debug)]
pub struct Author {
    first_name: String,
    last_name: String,
    email: String
}
    
#[derive(Debug)]
pub struct Post {
    pub id: Uuid,
    pub author: Author,
    pub creation_date: DateTime<Local>,
    pub title: String,
    // TODO add image support
    // pub icon: DynamicImage,
    pub body: String // TODO: update this with some kind of buffer
}

impl Post {
    pub fn new(
        author: Author, 
        title: String, 
        // icon: DynamicImage,
        body: String
    ) -> Post {

        Post {
            id: Uuid::new_v4(),
            creation_date: chrono::offset::Local::now(),
            author,
            title,
            // icon,
            body
        }
    }
}

#[derive(Debug)]
pub struct Gallery {
    pub count: i8,
    pub posts: Vec<Post>
}

impl Gallery {

    pub fn new() -> Gallery {
        Gallery {
            count: 0,
            posts: Vec::new()
        }

    }

    pub fn get_post(& self, uuid: Uuid) -> &Post{
        // TODO: Put db lookup logic here
        // TODO: actual error handling here

        let results:Vec<&Post> = self.posts
            .iter()
            .filter_map(|post| match post.id {
                uuid => Some(post),
                _ => None,
            })
            .collect();
        results[0]
    }
}



