#[macro_use]
extern crate webframework;

use webframework::{Database, Form, Former, Renderer, Tag, Content, Server, path::*, Store, Serialize, Error};
use std::sync::Arc;

#[derive(Default, Form, Store, Serialize, Clone)]
struct Bullet {
    #[id] #[form(type="text", min=1, max=512)] id: u64,
    #[form(name="Description", type="text", min=1, max=512)] description: String,
}

fn main() {
    let database = Arc::new(Database::new());
    let renderer = Renderer::new(database.clone());
    let mut server = Server::new("127.0.0.1:8000");
    server.threads(8);

    let bullet = Bullet {
        id: 0,
        description: String::from("This is a test.")
    };
    //bullet.create(&database);

    server.route(path!(/), renderer.content(move |arguments| html!(
        "<!DOCTYPE html>";
        html {
            head {
                title { "Hello Rust Web Framework!"; }
            }
            body {
                h1 { "Hello Rust Web Framework!"; }
                p { 
                    format!("{:?}", arguments);
                }
                Former::update(Bullet::read(0, &database.clone()).unwrap(), |bullet| {});
            }
        }
    )));
    
    server.start();
}