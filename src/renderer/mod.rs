#[macro_use]
mod content;
mod former;

pub use content::{Content, Form, Tag};
pub use former::Former;
use super::{Database, Store, Serialize, Error};
use super::{Request, Response, Arguments, Status, Method};
use std::path::Path;
use std::fs;
use rand::Rng; 
use rand::distributions::Alphanumeric;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Default, Debug, Store, Serialize)]
struct Session {
    #[id] id: String
}

impl Session {
    pub fn new(id: String) -> Session {
        Session {
            id,
            ..
            Default::default()
        }
    }
}

pub struct Renderer {
    database: Arc<Database>
}

impl Renderer {
    pub fn new(database: Arc<Database>) -> Renderer {
        Renderer {
            database
        }
    }

    pub fn content<C>(&self, content: C) -> impl Fn(Request, Arguments) -> Response + 'static
        where C: Fn(Arguments) -> Vec<Box<dyn Content>> + 'static
    {   
        let database = self.database.clone();

        move |request, arguments| {
            let cookie = request.fields.get("Cookie");
            let session_id = match cookie {
                Some(cookie) => {
                    match cookie
                        .clone()
                        .split(';')
                        .map(|element| {
                            let split = element.find('=').unwrap_or(element.len());
                            let (key, value) = element.split_at(split);
                            (key.trim(), value.split_at(1).1.trim())
                        })
                        .collect::<HashMap<&str, &str>>()
                        .get("session-id")
                    {
                        Some(id) => {
                            String::from(*id)
                        },
                        None => {
                            rand::thread_rng()
                                .sample_iter(&Alphanumeric)
                                .take(64)
                                .collect::<String>()
                        }
                    }
                },
                None => {
                    rand::thread_rng()
                        .sample_iter(&Alphanumeric)
                        .take(64)
                        .collect::<String>()
                }
            };
            
            let session = match Session::read(session_id.clone(), &database) {
                Ok(session) => session,
                Err(_) => {
                    let session = Session::new(session_id.clone());
                    session.create(&database).ok();
                    session
                }
            };
            
            let content = content(arguments);

            Response::new()
                .status(Status::OK)
                .field("Content-Type", "text/html; charset=utf-8")
                .field("Set-Cookie", &format!("session-id={}; Max-Age={}", session_id, 60*60*24*7))
                .content(match request.method {
                    Method::GET => {
                        let mut output = Vec::new();
                        for element in &content {
                            output.append(&mut element.get(&database));
                        }
                        output
                    },
                    Method::POST => {
                        let mut output = Vec::new();
                        for element in &content {
                            output.append(&mut element.get(&database));
                        }
                        output 
                    }
                })
        }
    }

    pub fn directory(&self, path: &'static str) -> impl Fn(Request, Arguments) -> Response {   
        move |_request, arguments| {
            let formatted = format!("{}{}", path, arguments.into_iter().map(|segment| format!("/{}", segment)).collect::<String>());
            let path = Path::new(&formatted);
            match fs::read(path) {
                Ok(data) => {
                    Response::new().status(Status::OK).field("Content-Type", "text/html; charset=utf-8").content(data)
                }
                Err(_) => {
                    Response::new().status(Status::NotFound)
                }
            }
        }
    }
}