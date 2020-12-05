use std::fs;

use super::http::Method;
use super::http::Request;
use super::http::Response;
use super::http::StatusCode;
use super::server::Handler;

pub struct WebsiteHandler {
    public_path: String
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        let acceptable_prefix = fs::canonicalize(&self.public_path).unwrap();
        dbg!(&acceptable_prefix);

        match fs::canonicalize(path) {
            Ok(path) => {
                dbg!(&path);
                if path.starts_with(acceptable_prefix) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("attempted directory traversal attack: {}", file_path);
                    None
                }
            }
            Err(_) => None
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(
                    StatusCode::Ok,
                    self.read_file("index.html")
                ),
                path => match self.read_file(path) {
                    Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
                    None => Response::new(StatusCode::NotFound, None)
                }
            }
            _ => Response::new(StatusCode::NotFound, None)
        }
    }
}