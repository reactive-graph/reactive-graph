use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodeFormatterError {
    #[error("\nError: Failed to parse source code:\n{0}\n\n=====\n{1}")]
    ParserError(String, String),
}
