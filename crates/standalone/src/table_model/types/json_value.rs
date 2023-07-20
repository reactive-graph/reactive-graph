use serde_json::Value;

pub fn pretty_json(value: &Value) -> String {
    serde_json::to_string_pretty(value).unwrap_or(String::from("JSON Error"))
}
