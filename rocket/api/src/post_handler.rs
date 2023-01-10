use rocket::{get, post};
use rocket::response::content;
use rocket::response::status::{Created, NotFound};
use rocket::serde::json::Json;
use rocket::{State};
use application::post::{create, read};
use domain::models::{NewPost, Post};
use shared::response_dto::{FileSystem, FileSystemReq, Response, ResponseBody};
use crate::state::visit_counter::VisitorCounter;

// rank = If we have multiple routes handling the same path, then Rocket will rank the functions and start checking from the rank with the lowest number.
/*
#[post("/post", data = "<data>")] <-- body
fn post(data: Form<Filters>) -> &'static str {
 */

#[get("/rust-filesystem-read?<extra..>", rank = 1)] //
pub fn get_filesystem_handler(extra: FileSystemReq) -> Json<FileSystem>  {
    let json_file = std::fs::read_to_string("sample.json").unwrap();
    let mut json: FileSystem = serde_json::from_str::<FileSystem>(&json_file).unwrap();
    json.extra = Some(extra.extra);

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

#[get("/rust-filesystem-read-two?<extra>")]
pub fn get_filesystem_handler_two(extra: String) -> Json<FileSystem>  {
    if extra.len() > 100 {
        panic!("to big...");
    }
    let json_file = std::fs::read_to_string("sample.json").unwrap();
    let mut json: FileSystem = serde_json::from_str::<FileSystem>(&json_file).unwrap();
    json.extra = Some(extra);

    Json(json)
}

#[post("/new_post", format = "application/json", data = "<post>")]
pub fn create_post_handler(counter: &State<VisitorCounter>, post: Json<NewPost>) -> content::RawJson<Created<String>> { // Result<&User, NotFound<&str>> {
    counter.increment();
    create::create_post(post) //  user.ok_or(NotFound("User not found"))
}

