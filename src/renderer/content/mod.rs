mod form;
#[macro_use]
mod tag;

pub use form::Form;
pub use form_derive::Form;
pub use tag::Tag;

use database::Database;

pub trait Content: Send + Sync {
    fn get(&self, _: &Database) -> Vec<u8>;
    fn post(&self, _: &Database) -> Vec<u8>;
}

impl Content for &str {
    fn get(&self, _: &Database) -> Vec<u8> {
        String::from(*self).into_bytes()
    }
    
    fn post(&self, _: &Database) -> Vec<u8> {
        Vec::new()
    }
}

impl Content for String {
    fn get(&self, _: &Database) -> Vec<u8> {
        self.clone().into_bytes()
    }
    
    fn post(&self, _: &Database) -> Vec<u8> {
        Vec::new()
    }
}