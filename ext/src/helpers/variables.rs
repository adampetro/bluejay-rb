use magnus::{RHash, Value};

pub trait Variables<const CONST: bool> {
    fn get(&self, key: &str) -> Option<Value>;
}

impl Variables<false> for RHash {
    fn get(&self, key: &str) -> Option<Value> {
        RHash::get(*self, key)
    }
}

impl Variables<true> for () {
    fn get(&self, _: &str) -> Option<Value> {
        None
    }
}
