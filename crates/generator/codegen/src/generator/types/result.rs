use std::path::PathBuf;

#[derive(Debug)]
pub struct CodeGenerationResult {
    /// The absolute path to the generated code file.
    pub path: PathBuf,
}
