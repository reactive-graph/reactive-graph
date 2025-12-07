pub use config::*;
pub use formatter::CodeFormatter;
pub use formatter::CodeFormatterError;
pub use fs::*;
pub use generator::*;
pub use targets::*;

pub mod config;
pub mod formatter;
pub mod fs;
pub mod generator;
pub mod targets;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
