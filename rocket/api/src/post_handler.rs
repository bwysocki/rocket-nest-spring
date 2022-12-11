use shared::response_dto::{Response, ResponseBody, FileSystem, FileSystemReq};
use application::post::{read};
use rocket::response::content;
use domain::models::{Post};
use rocket::{get};
use rocket::response::status::{NotFound};
use rocket::serde::json::Json;

#[get("/rust-filesystem-read?<extra>")]
pub fn get_filesystem_handler(extra: String) -> Json<FileSystem>  {
    if extra.len() > 100 {
        panic!("to big...");
    }
    let json_file = std::fs::read_to_string("sample.json").unwrap();
    let mut json: FileSystem = serde_json::from_str::<FileSystem>(&json_file).unwrap();
    json.extra = Some(extra);

    Json(json)
}

#[get("/")]
pub fn list_posts_handler() -> String {
    let posts: Vec<Post> = read::list_posts();
    let response = Response { body: ResponseBody::Posts(posts) };

    serde_json::to_string(&response).unwrap()
}

#[get("/post/<post_id>")]
pub fn list_post_handler(post_id: i32) -> Result<String, NotFound<String>> {
    let post = read::list_post(post_id)?;
    let response = Response { body: ResponseBody::Post(post) };

    Ok(serde_json::to_string(&response).unwrap())
}