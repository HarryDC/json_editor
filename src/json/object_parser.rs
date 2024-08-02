
use super::error::Error;
use super::error::Error::{EndOfLine};
use super::Object;
use super::value::JsonValueType;
use super::value::JsonValueType::{JsonTypeArray, JsonTypeBool, JsonTypeNull, JsonTypeNumber, JsonTypeObject, JsonTypeString};
use super::state::State;



pub(crate) fn parse_value(state : &mut State) -> Result<JsonValueType, Error> {
    if state.skip_whitespace() {
        return Err(EndOfLine)
    }

    return match state.peek() {
        Some(&'t') | Some(&'f') => {
            if state.read_literal("true") {Ok(JsonTypeBool(true))}
            else if state.read_literal("false") {Ok(JsonTypeBool(false))}
            else { Err(state.error()) }
        },
        Some(&'n') => {
            if state.read_literal("null") {Ok(JsonTypeNull)}
            else { Err(state.error())}
        }
        Some(&'[') => {
            state.take();
            parse_array(state)
        },
        Some(&'"') => {
            state.take();
            parse_string(state)
        },
        Some(&'{') => {
            state.take();
            parse_object(state)
        }
        _ => parse_number(state)
    }
}

fn parse_object(state: &mut State) -> Result<JsonValueType, Error> {

    let mut result = Object::new();

    if state.read_char('}') {
        return Ok(JsonTypeObject(result))
    }

    loop {
        let key = parse_value(state);
        let key_string: String;
        match key {
            Ok(JsonTypeString(val)) => key_string = val,
            _ => return Err(state.error())
        }
        if !state.read_char(':') { return Err(state.error())}
        let value = parse_value(state);
        if value.is_err() {return Err(state.error());}
        result.insert(key_string, value.unwrap());
        if state.read_char('}') {break;}
        else if state.read_char(',') {continue;}
        else {return Err(state.error())}
    }

    Ok(JsonTypeObject(result))

}

fn parse_array(state: &mut State) -> Result<JsonValueType, Error> {

    let mut vec = Vec::new();

    // Empty Vector
    if state.peek().is_some_and(|x| x == &']' ) {
        state.take();
        return Ok(JsonTypeArray(vec));
    }

    loop {
        let value = parse_value(state)?;
        vec.push(value);
        if state.skip_whitespace() { return Err(EndOfLine) }

        match state.take() {
            Some(&',') => continue,
            Some(&']') => break,
            _ => return Err(state.error())
        }
    }

    return Ok(JsonTypeArray(vec));
}

fn parse_number(state: &mut State) -> Result<JsonValueType, Error> {
    let valid = vec![
        '0','1','2','3','4','5','6','7','8','9','-','e','E','.'
    ];

    let mut value = String::new();

    loop {
        match state.peek() {
            None => break,

            x => if !valid.contains(x.unwrap()) {
                break;
            } else {
                value.push(*x.unwrap());
                state.take();
            }
        }
    }

    let num = value.parse::<f64>();
    if num.is_err() {
        Err(state.error())
    } else {
        Ok(JsonTypeNumber(num.unwrap()))
    }

}

fn parse_string(state: &mut State) -> Result<JsonValueType, Error> {
    let mut result = String::new();
    let mut slash = false;
    loop {
        match state.take() {
            Some(&'\\') => {
                if slash {
                    result.push('\\');
                    slash = false;
                } else {
                    slash = true;
                }
            },
            Some(&'"') => {
                if slash {
                    result.push('"');
                    slash = false;
                } else {
                    break;
                }
            }
            Some(x) => {
                slash = false;
                result.push(*x);
            },
            None => return Err(EndOfLine)
        }
    }
    return Ok(JsonTypeString(result));
}


