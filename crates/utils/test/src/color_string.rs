use colored::ColoredString;
use colored::Colorize;

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
