use std::sync::atomic::{AtomicUsize, Ordering};

use rocket::{Request, Data, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Method};

pub struct Counter {
    get: AtomicUsize,
    post: AtomicUsize,
}

impl Counter{
    pub fn new() -> Counter {
        Counter {
            get: AtomicUsize::new(1),
            post: AtomicUsize::new(1)
        }
    }
}

#[rocket::async_trait]
impl Fairing for Counter {
    // This is a request and response fairing named "GET/POST Counter".
    fn info(&self) -> Info {
        Info {
            name: "GET/POST Counter",
            kind: Kind::Request | Kind::Response
        }
    }

    // Increment the counter for `GET` and `POST` requests.
    async fn on_request(&self, request: &mut Request<'_>, _: &mut Data<'_>) {
        match request.method() {
            Method::Get => self.get.fetch_add(1, Ordering::Relaxed),
            Method::Post => self.post.fetch_add(1, Ordering::Relaxed),
            _ => return
        };
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, _response: &mut Response<'r>) {
        let get_count = self.get.load(Ordering::Relaxed);
        let post_count = self.post.load(Ordering::Relaxed);
        println!("Get: {}\nPost: {}", get_count, post_count);
    }
}