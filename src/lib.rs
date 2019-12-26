#[cfg_attr(test, macro_use)] 
extern crate httpserver;

#[macro_use]
pub mod renderer;

pub use database;

#[cfg(test)]
mod tests {

    use super::renderer::{Renderer, Form, Tag, Content};
    use super::database::Database;
    use super::httpserver::{Server, path::*};
    

    #[derive(Default, Form)]
    struct User {
        #[form(name="Username", type="text", min=4, max=16)] username: String,
        #[form(name="Password", type="password", min=4, max=16)] password: String,
        #[form(name="Repeat Password", type="password")] password_repeat: String,
        #[form(name="Age", type="number", min=0, max=200)] age: u8
    }

    #[test]
    fn initialize_server() {
        let database = Database::new();
        let renderer = Renderer::new(&database);
        let mut server = Server::new("127.0.0.1:8000");
        server.threads(8);

        server.route(path!(/), renderer.content(html!(
            "<!DOCTYPE html>";
            html [
                head [
                    title [ "Hello Rust Web Framework!"; ]
                ]
                body [
                    h1 [ "Hello Rust Web Framework!"; ]
                ]
            ]
        )));
        
        server.start();
    }
}
