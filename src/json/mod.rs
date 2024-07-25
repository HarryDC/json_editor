use error::Error;


mod object_parser;
mod state;
pub mod value;
pub mod error;

pub fn to_object(content: &str) -> Result<value::JsonValueType, Error> {
    let mut state = state::State::new(content);
    object_parser::parse_value(&mut state)
}