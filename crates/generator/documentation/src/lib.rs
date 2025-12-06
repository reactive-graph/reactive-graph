pub use fs::DOCS_DIRECTORY;
pub use fs::create_and_get_documentation_path;
pub use fs::get_workspace_root_docs_path;
pub use types::GenerateDocumentation;
pub use types::collections::GenerateDocumentations;
pub use types::collections::GenerateTypeSystemDocumentations;
pub use types::config::DocumentationConfig;
pub use types::config::DocumentationConfigPreset;
pub use types::config::FromDocumentationConfigPreset;

pub mod error;
pub mod fs;
pub mod generator;
pub mod markdown;
pub mod types;

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
