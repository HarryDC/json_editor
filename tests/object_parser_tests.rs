use std::collections::HashMap;
use json_editor::json::error::Error::EndOfLine;
use json_editor::json::{Array, Object, to_object};
use json_editor::json::value::JsonValueType;
use json_editor::json::value::JsonValueType::{JsonTypeArray, JsonTypeBool, JsonTypeNumber, JsonTypeObject, JsonTypeString};

#[test]
fn test_parse_bool() {
    assert_eq!(to_object("true"), Ok(JsonTypeBool(true)));
    assert_eq!(to_object("false"), Ok(JsonTypeBool(false)));
}
#[test]
fn test_parse_array() {
    assert_eq!(to_object("[]"), Ok(JsonTypeArray(Array(Vec::new()))));
    assert_eq!(to_object("[true,false]"), Ok(JsonTypeArray(Array(vec![JsonTypeBool(true), JsonTypeBool(false)]))));
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
    let pair_1 = ("one".to_string(), JsonTypeNumber(1.0));
    let pair_2 = ("two".to_string(), JsonTypeNumber(2.0));
    assert_eq!(to_object("{}"), Ok(JsonTypeObject(Object(HashMap::new()))));
    assert_eq!(to_object("{\"one\" : 1}"), Ok(JsonTypeObject(Object(HashMap::from([pair_1.clone()])))));
    assert_eq!(to_object("{\"one\" : 1, \"two\":2}"), Ok(JsonTypeObject(Object(HashMap::from([pair_1.clone(), pair_2.clone()])))));
}

#[test]
fn test_array_value_to_string() {
    {
        let json = r#"[]
"#;

        let converted = to_object(json).unwrap();
        let result = converted.to_string();
        assert_eq!(json, result);
    }
    {
        let json = r#"["one", "two", "three"]
"#;

        let converted = to_object(json).unwrap();
        let result = converted.to_string();
        assert_eq!(json, result);
    }
}
#[test]
fn test_object_value_to_string() {
    {
        let json = r#"{}
"#;

        let converted = to_object(json).unwrap();
        let result = converted.to_string();
        assert_eq!(json, result);
    }
    // Fix later
//     {
//         let json = r#"{"three" : 1.1, "two" : "value", "one" : true}
// "#;
//         let expected = vec!["1.1", "value", "true"];
//         let keys  = vec!["three", "two", "one"];
//
//         let converted = to_object(json).unwrap();
//         let result = converted.to_string();
//         // Can't check against string as ordering changes due to hash map
//
//         if let Some(obj) = match converted {
//             JsonTypeObject(obj) => {Some(obj)},
//             _ => None
//         } {
//             assert_eq!(obj.len(), 3)
//
//         } else {
//             assert!(false)
//         }
//
//     }
}



