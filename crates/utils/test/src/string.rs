use convert_case::Case;
use convert_case::Casing;
use random_string::generate;
use serde_json::Value;
use serde_json::json;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_LOWERCASE_LETTERS: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_lowercase_string() -> String {
    generate(10, CHARSET_LOWERCASE_LETTERS).to_string()
}

pub fn r_namespace_segment() -> String {
    let mut path_segment = String::new();
    for _ in 1..3 {
        path_segment = format!("{} {}", path_segment, r_lowercase_string());
    }
    path_segment
}

pub fn r_namespace_path_segment() -> String {
    r_namespace_segment().to_case(Case::Snake)
}

pub fn r_namespace_type_name() -> String {
    r_namespace_segment().to_case(Case::Pascal)
}

pub fn r_string_255() -> String {
    generate(255, CHARSET_LETTERS).to_string()
}

pub fn r_string_1000() -> String {
    generate(1000, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}
