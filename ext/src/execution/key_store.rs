use std::collections::HashMap;
use std::cell::RefCell;
use magnus::{RString, RArray};

pub(super) struct KeyStore<'a> {
    hash_map: RefCell<HashMap<&'a str, RString>>,
    strings: RArray,
}

impl<'a> KeyStore<'a> {
    pub fn new() -> Self {
        Self { hash_map: RefCell::new(HashMap::new()), strings: RArray::new() }
    }

    pub fn get(&self, s: &'a str) -> RString {
        *self.hash_map.borrow_mut().entry(s).or_insert_with(|| {
            let s: RString = s.into();
            s.freeze();
            self.strings.push(s).unwrap();
            s
        })
    }
}