use json_parser::json::error::Error::EndOfLine;
use json_parser::json::to_object;
use json_parser::json::value::JsonValueType::{JsonTypeArray, JsonTypeBool, JsonTypeNumber, JsonTypeObject, JsonTypeString};

#[test]
fn test_parse_bool() {
    assert_eq!(to_object("true"), Ok(JsonTypeBool(true)));
    assert_eq!(to_object("false"), Ok(JsonTypeBool(false)));
}
#[test]
fn test_parse_array() {
    assert_eq!(to_object("[]"), Ok(JsonTypeArray(Vec::new())));
    assert_eq!(to_object("[true,false]"), Ok(JsonTypeArray(vec![JsonTypeBool(true), JsonTypeBool(false)])));
    assert_eq!(to_object("[[]"), Err(EndOfLine));
}
#[test]
fn test_json_parse_string() {
    assert_eq!(to_object("\"\""), Ok(JsonTypeString(String::from(""))));
    assert_eq!(to_object("\"test\""), Ok(JsonTypeString(String::from("test"))));
    assert_eq!(to_object("\"test"), Err(EndOfLine));
    assert_eq!(to_object("\"t\\\"est\""), Ok(JsonTypeString(String::from("t\"est"))));
}

#[test]
fn test_json_parse_number() {
    assert_eq!(to_object("1"), Ok(JsonTypeNumber(1.0)));
    assert_eq!(to_object("2.5"), Ok(JsonTypeNumber(2.5)));
    assert_eq!(to_object("-5.0"), Ok(JsonTypeNumber(-5.0)));
}

#[test]
fn test_json_parse_object() {
    let pair_1 = (JsonTypeString("one".parse().unwrap()), JsonTypeNumber(1.0));
    let pair_2 = (JsonTypeString("two".parse().unwrap()), JsonTypeNumber(2.0));
    assert_eq!(to_object("{}"), Ok(JsonTypeObject(Vec::new())));
    assert_eq!(to_object("{\"one\" : 1}"), Ok(JsonTypeObject(Vec::from([pair_1.clone()]))));
    assert_eq!(to_object("{\"one\" : 1, \"two\":2}"), Ok(JsonTypeObject(Vec::from([pair_1.clone(), pair_2.clone()]))));
}

#[test]
fn test_json_parse_file() {

}



