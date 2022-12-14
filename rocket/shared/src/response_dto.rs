use domain::models::Post;
use rocket::serde::{Deserialize, Serialize};
//use validator::{Validate};
use rocket_validation::{Validate};
use rocket::form::FromForm;

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