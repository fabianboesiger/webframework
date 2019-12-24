#[macro_use]
mod content;

pub use content::{Content, Form, Tag};
use super::database::{Database, Store, Serialize, Error};
use super::httpserver::{Request, Response, Arguments, Status, Method};

#[derive(Default, Store, Serialize)]
struct Session {
    #[id] id: String
}


pub struct Renderer<'a> {
    database: &'a Database
}

impl Renderer<'_> {
    pub fn new<'a>(database: &'a Database) -> Renderer<'a> {
        Renderer {
            database
        }
    }

    pub fn html(&self, content: Vec<Box<dyn Content>>) -> impl Fn(Request, Arguments) -> Response {        
        move |request, arguments| {
            Response::new().status(Status::OK).content(match request.method {
                Method::GET => {
                    let mut output = String::from("<!DOCTYPE html>").into_bytes();
                    for element in &content {
                        output.append(&mut element.get());
                    }
                    output
                },
                Method::POST => {
                    let mut output = Vec::new();
                    for element in &content {
                        output.append(&mut element.get());
                    }
                    output
                }
            })
        }
    }
}