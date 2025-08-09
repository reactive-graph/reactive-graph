use convert_case::Case;
use convert_case::Casing;
use random_string::generate;
use serde_json::Value;
use serde_json::json;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const CHARSET_NAMESPACE_PATH_SEGMENT: &str = "abcdefghijklmnopqrstuvwxyz          ";
const CHARSET_NAMESPACE_TYPE_NAME: &str = "abcdefghijklmnopqrstuvwxyz          ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_namespace_path_segment() -> String {
    generate(20, CHARSET_NAMESPACE_PATH_SEGMENT).to_string().to_case(Case::Snake)
}

pub fn r_namespace_type_name() -> String {
    generate(20, CHARSET_NAMESPACE_TYPE_NAME).to_string().to_case(Case::Pascal)
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
