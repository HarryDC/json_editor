use std::collections::HashMap;
use error::Error;
use crate::json::value::JsonValueType;

mod object_parser;
mod state;
pub mod value;
pub mod error;

pub type Array = Vec<JsonValueType>;
pub type Object = HashMap<String, JsonValueType>;

pub fn to_object(content: &str) -> Result<value::JsonValueType, Error> {
    let mut state = state::State::new(content);
    object_parser::parse_value(&mut state)
}