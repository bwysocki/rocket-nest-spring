use rocket::Request;
use rocket::catch;

#[catch(404)]
pub fn not_found(req: &Request) -> String {
    format!("Sorry! We cannot find this page {}.", req.uri())
}