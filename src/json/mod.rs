use std::collections::HashMap;
use std::hash::Hasher;
use error::Error;
use crate::json::value::JsonValueType;

mod object_parser;
mod state;
pub mod value;
pub mod error;

#[derive(Debug, PartialEq, Clone)]
pub struct Array(pub Vec<JsonValueType>);

impl std::hash::Hash for Array {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = self as *const Array;
        let addr = ptr as usize;
        state.write_usize(addr);
    }
}
#[derive(Debug, PartialEq, Clone)]
pub struct Object(pub HashMap<String, JsonValueType>);

impl std::hash::Hash for Object {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let ptr = self as *const Object;
        let addr = ptr as usize;
        state.write_usize(addr);
    }
}

pub fn to_object(content: &str) -> Result<value::JsonValueType, Error> {
    let mut state = state::State::new(content);
    object_parser::parse_value(&mut state)
}