use crate::parse_all::Error::{Character, EndOfLine};
use crate::parse_all::JsonValueType::{JsonTypeArray, JsonTypeBool, JsonTypeNull, JsonTypeNumber, JsonTypeObject, JsonTypeString};


struct State {
    content: Vec<char>,
    cursor: usize
}

impl State {
    pub fn new(string: &str) -> Self {
        Self {
            cursor: 0,
            content: string.chars().collect(),
        }
    }

    pub fn error(&self) -> Error {
        if self.at_end() {
            EndOfLine
        } else {
            Character(self.cursor)
        }
    }

    pub fn at_end(&self) -> bool {
        self.cursor >= self.content.len()
    }

    pub fn cursor(&self) -> usize {
        self.cursor
    }

    pub fn peek(&self) -> Option<&char> {
        self.content.get(self.cursor)
    }

    pub fn take(&mut self) -> Option<&char> {
        match self.content.get(self.cursor) {
            Some(character) => {
                self.cursor += 1;
                Some(character)
            }
            None => None,
        }
    }

    // Return true if we're at the end of the string
    pub fn skip_whitespace(&mut self) -> bool {
        while self.peek().is_some_and(|x| x.is_whitespace()) {
            self.take();
        }
        self.at_end()
    }

    pub fn read_char(&mut self, c : char) -> bool {
        let mut result = false;
        if !self.skip_whitespace() && self.peek() == Some(&c) {
            result = true;
            self.cursor += 1
        }
        result
    }

    pub fn read_literal(&mut self, lit : &str) -> bool {
        if self.skip_whitespace() {
            return false;
        }

        for c in lit.chars() {
            if self.peek() != Some(&c) {
                return false;
            }
            self.cursor+=1;
        }
        true
    }
}


#[derive(Debug, PartialEq, Clone)]
enum JsonValueType {
    JsonTypeNull,
    JsonTypeBool(bool),
    JsonTypeNumber(f64),
    JsonTypeObject(Vec<(JsonValueType,JsonValueType)>), // Is a vector with pairwise entries, key, value
    JsonTypeArray(Vec<JsonValueType>), // Is a vector, all entries are plain
    JsonTypeString(String)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Error {
    /// An unexpected end-of-line has been found.
    EndOfLine,

    /// A syntax error at the indicated cursor position has been found.
    Character(usize),
}

fn json_parse_value(state : &mut State) -> Result<JsonValueType, Error> {
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
            json_parse_array(state)
        },
        Some(&'"') => {
            state.take();
            json_parse_string(state)
        },
        Some(&'{') => {
            state.take();
            json_parse_object(state)
        }
        _ => json_parse_number(state)
    }
}

fn json_parse_object(state: &mut State) -> Result<JsonValueType, Error> {

    let mut result = Vec::new();

    if state.read_char('}') {
        return Ok(JsonTypeObject(result))
    }

    loop {
        let key = json_parse_value(state);
        match key {
            Ok(JsonTypeString(_)) => (),
            _ => return Err(state.error())
        }
        if !state.read_char(':') { return Err(state.error())}
        let value = json_parse_value(state);
        if value.is_err() {return Err(state.error());}
        result.push((key.unwrap(),value.unwrap()));
        if state.read_char('}') {break;}
        else if state.read_char(',') {continue;}
        else {return Err(state.error())}
    }

    Ok(JsonTypeObject(result))

}

fn json_parse_bool(state : &mut State) -> Result<JsonValueType, Error> {
    return if state.content[state.cursor..state.cursor + 3] == ['r', 'u', 'e'] {
        state.cursor += 3;
        Ok(JsonTypeBool(true))
    } else if state.content[state.cursor..state.cursor + 4] == ['a', 'l', 's', 'e'] {
        state.cursor += 4;
        Ok(JsonTypeBool(false))
    } else { Err(Character(state.cursor)) }
}

fn json_parse_array(state: &mut State) -> Result<JsonValueType, Error> {

    let mut vec = Vec::new();

    // Empty Vector
    if state.peek().is_some_and(|x| x == &']' ) {
        state.take();
        return Ok(JsonTypeArray(vec));
    }

    loop {
        let value = json_parse_value(state)?;
        vec.push(value);
        if state.skip_whitespace() { return Err(EndOfLine) }

        match state.take() {
            Some(&',') => continue,
            Some(&']') => break,
            _ => return Err(Character(state.cursor))
        }
    }

    return Ok(JsonTypeArray(vec));
}

fn json_parse_number(state: &mut State) -> Result<JsonValueType, Error> {
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
        Err(Character(state.cursor))
    } else {
        Ok(JsonTypeNumber(num.unwrap()))
    }

}

fn json_parse_string(state: &mut State) -> Result<JsonValueType, Error> {
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

fn json_parse_all(content: &str) -> Result<JsonValueType, Error> {
    let mut state = State::new(content);
    json_parse_value(&mut state)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_state_read_char() {
        let mut state_1 = State::new(" te ");
        assert_eq!(true, state_1.read_char('t'));
        assert_eq!(false, state_1.read_char('t'));
        assert_eq!(true, state_1.read_char('e'));
        assert_eq!(false, state_1.read_char('e'));
        assert_eq!(true, state_1.at_end());
    }

    #[test]
    fn test_state_read_literal() {
        let mut state_1 = State::new("  true ");
        assert_eq!(true, state_1.read_literal("true"));


        let mut state_2 = State::new("  true ");
        assert_eq!(false, state_2.read_literal("trx"));

        let mut state_3 = State::new("false  true ");
        assert_eq!(true, state_3.read_literal("false"));
        assert_eq!(true, state_3.read_literal("true"));
    }

    #[test]
    fn test_state_skip_whitespace() {
        let mut state_1 = State::new("test");
        assert_eq!(false, state_1.skip_whitespace());
        assert_eq!(0, state_1.cursor);
        assert_eq!(false, state_1.at_end());
        assert_eq!(Some(&'t'), state_1.peek());

        let mut state_2 = State::new("   test");
        assert_eq!(false, state_2.skip_whitespace());
        assert_eq!(3, state_2.cursor);
        assert_eq!(false, state_2.at_end());
        assert_eq!(Some(&'t'), state_2.peek());

        let mut state_3 = State::new("t    ");
        assert_eq!(Some(&'t'), state_3.take());
        assert_eq!(true, state_3.skip_whitespace());
        assert_eq!(true, state_3.at_end());
    }
    #[test]
    fn test_parse_bool() {
        assert_eq!(json_parse_all("true"), Ok(JsonTypeBool(true)));
        assert_eq!(json_parse_all("false"), Ok(JsonTypeBool(false)));
    }
    #[test]
    fn test_parse_array() {
        assert_eq!(json_parse_all("[]"), Ok(JsonTypeArray(Vec::new())));
        assert_eq!(json_parse_all("[true,false]"), Ok(JsonTypeArray(vec![JsonTypeBool(true),JsonTypeBool(false)])));
        assert_eq!(json_parse_all("[[]"), Err(EndOfLine));
    }
    #[test]
    fn test_json_parse_string() {
        assert_eq!(json_parse_all("\"\""), Ok(JsonTypeString(String::from(""))));
        assert_eq!(json_parse_all("\"test\""), Ok(JsonTypeString(String::from("test"))));
        assert_eq!(json_parse_all("\"test"), Err(EndOfLine));
        assert_eq!(json_parse_all("\"t\\\"est\""), Ok(JsonTypeString(String::from("t\"est"))));
    }

    #[test]
    fn test_json_parse_number() {
        assert_eq!(json_parse_all("1"), Ok(JsonTypeNumber(1.0)));
        assert_eq!(json_parse_all("2.5"), Ok(JsonTypeNumber(2.5)));
        assert_eq!(json_parse_all("-5.0"), Ok(JsonTypeNumber(-5.0)));
    }

    #[test]
    fn test_json_parse_object() {
        let pair_1 = (JsonTypeString("one".parse().unwrap()), JsonTypeNumber(1.0));
        let pair_2 = (JsonTypeString("two".parse().unwrap()), JsonTypeNumber(2.0));
        assert_eq!(json_parse_all("{}"), Ok(JsonTypeObject(Vec::new())));
        assert_eq!(json_parse_all("{\"one\" : 1}"), Ok(JsonTypeObject(Vec::from([pair_1.clone()]))));
        assert_eq!(json_parse_all("{\"one\" : 1, \"two\":2}"), Ok(JsonTypeObject(Vec::from([pair_1.clone(), pair_2.clone()]))));
    }
}
