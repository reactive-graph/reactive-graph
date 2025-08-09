use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;

pub mod collections;
pub mod components;
pub mod entities;
pub mod flows;
pub mod relations;

pub trait GenerateDocumentation<TY> {
    fn generate_documentation(&self) -> Result<MarkdownDocumentation<TY>, DocumentationGenerationError>;
}
