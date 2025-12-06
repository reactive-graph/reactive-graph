use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::TypeResolver;

pub mod collections;
pub mod components;
pub mod config;
pub mod entities;
pub mod extensions;
pub mod flows;
pub mod properties;
pub mod relations;
pub mod system;

pub trait GenerateDocumentation<TY> {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<TY>, DocumentationGenerationError>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {}
}
