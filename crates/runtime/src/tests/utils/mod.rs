use colored::ColoredString;
use colored::Colorize;
use random_string::generate;
use serde_json::json;
use serde_json::Value;

const CHARSET_LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

pub fn r_string() -> String {
    generate(10, CHARSET_LETTERS).to_string()
}

pub fn r_json_string() -> Value {
    json!(r_string())
}

pub fn verb(s: &str) -> String {
    format!("{:>12}", s.green().bold())
}

pub fn error_verb(s: &str) -> String {
    format!("{:>12}", s.red().bold())
}

pub fn status_ok() -> ColoredString {
    "✓".green()
}

pub fn status_failed() -> ColoredString {
    "❌".red()
}
