use crate::json::{Array, Object};

#[derive(Debug, PartialEq, Clone)]
pub enum JsonValueType {
    JsonTypeNull,
    JsonTypeBool(bool),
    JsonTypeNumber(f64),
    JsonTypeObject(Object), // Is a vector with pairwise entries, key, value
    JsonTypeArray(Array), // Is a vector, all entries are plain
    JsonTypeString(String)
}

impl JsonValueType {

    pub fn as_bool(&self) -> Option<bool> {
        match self {
            JsonValueType::JsonTypeBool(val) => Option::from(*val),
            _ => None
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            JsonValueType::JsonTypeString(val) => Option::from(val),
            _ => None
        }
    }

    pub fn as_number(&self) -> Option<f64> {
        match self {
            JsonValueType::JsonTypeNumber(val) => Option::from(*val),
            _ => None
        }
    }

    pub fn is_null(&self) -> bool {
        match self {
            JsonValueType::JsonTypeNull => true,
            _ => false
        }
    }

    fn as_array(&self) -> Option<&Array> {
        match self {
            JsonValueType::JsonTypeArray(val) => Option::from(val),
            _ => None
        }
    }

    fn as_object(&self) -> Option<&Object> {
        match self {
            JsonValueType::JsonTypeObject(val) => Option::from(val),
            _ => None
        }
    }

    pub fn len(&self) -> Option<usize> {
        match self {
            JsonValueType::JsonTypeObject(vec) => Option::from(vec.len()),
            JsonValueType::JsonTypeArray(vec) => Option::from(vec.len()),
            _ => None
        }
    }

    pub fn to_string(&self) -> String {
        match &self {
            JsonValueType::JsonTypeNull => "null".to_string(),
            JsonValueType::JsonTypeBool(val) => {
                if *val { "true".to_string() }
                else { "false".to_string() }
            }
            JsonValueType::JsonTypeNumber(val) => { val.to_string() }
            JsonValueType::JsonTypeObject(val) => {
                    let mut result = "{".to_string();
                    for (index,(key,value)) in val.iter().enumerate() {
                        result = format!("{}\"{}\" : {}",result, key, value.to_string());
                        if index < val.len() - 1 {
                            result += ", ";
                        }
                    }
                result + "}\n"
            }
            JsonValueType::JsonTypeArray(val) => {
                    let mut result : String = "[".to_string();
                    for (i, item) in val.iter().enumerate()
                    {
                        if i != 0 {
                            result = format!("{}, {}", result, item.to_string());
                        } else {
                            result = format!("{}{}", result, item.to_string());
                        }
                    }

                    result + "]\n"
            }
            JsonValueType::JsonTypeString(val) => { format!("\"{}\"", val) }
        }
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_to_string_array() {

    }
}