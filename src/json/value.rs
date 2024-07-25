#[derive(Debug, PartialEq, Clone)]
pub enum JsonValueType {
    JsonTypeNull,
    JsonTypeBool(bool),
    JsonTypeNumber(f64),
    JsonTypeObject(Vec<(JsonValueType,JsonValueType)>), // Is a vector with pairwise entries, key, value
    JsonTypeArray(Vec<JsonValueType>), // Is a vector, all entries are plain
    JsonTypeString(String)
}