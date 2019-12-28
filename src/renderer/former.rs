use super::{Form, Content};
use database::{Database, Store, Serialize};
use std::marker::PhantomData;

pub struct Former<S: Store + Form + Default + Send + Sync, F: Fn(S) + Send + Sync> {
    load: Option<S>,
    function: F
}

impl<S: Store + Form + Default + Send + Sync, F: Fn(S) + Send + Sync> Former<S, F> {
    pub fn update(object: S, function: F) -> Former<S, F> {
        Former {
            load: Some(object),
            function: function
        }
    }
}

impl<S: Store + Form + Default + Send + Sync, F: Fn(S) + Send + Sync> Content for Former<S, F> {
    fn get(&self, database: &Database) -> Vec<u8> {
        match &self.load {
            Some(object) => {
                object.html()
            },
            None => {
                S::default().html()
            }
        }
    }
    
    fn post(&self, _: &Database) -> Vec<u8> {
        Vec::new()
    }
}