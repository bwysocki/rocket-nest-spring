use rocket::form::FromForm;
use rocket::response::{Responder};
use rocket::serde::{Deserialize, Serialize};
use rocket_validation::Validate;

use domain::models::Post;

#[derive(Serialize, Deserialize, Validate, FromForm)]
pub struct FileSystemReq {
    #[validate(length(max = 10))]
    pub extra: String
}

#[derive(Serialize, Deserialize)]
pub struct FileSystem {
    pub fruit: String,
    pub size: String,
    pub color: String,
    pub price: f64,
    #[serde(rename = "longDesc")]
    pub long_desc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<String>,
}

#[derive(Serialize, Deserialize, Responder)]
pub struct CreatedPost {
    pub id: String,
}

impl CreatedPost {
    pub fn new(id: &str) -> CreatedPost {
        CreatedPost {
            id: id.to_string()
        }
    }
}

#[derive(Serialize)]
pub enum ResponseBody {
    Message(String),
    Post(Post),
    Posts(Vec<Post>),
    FileSystem(FileSystem)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Response {
    pub body: ResponseBody,
}