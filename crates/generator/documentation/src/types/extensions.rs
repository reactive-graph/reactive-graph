use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::Extension;
use reactive_graph_graph::Extensions;
use reactive_graph_graph::TypeResolver;

impl GenerateDocumentation<Extension> for Extension {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<Extension>, DocumentationGenerationError> {
        Ok(
            TypedMarkdownDocumentation::new(self.clone())
                .ty(config)
                // TODO: If extension has an entity type, generate or link documentation for the entity type.
                // entity_ty(config)
                .description(config),
            // TODO: Implement json schema for the extension
            // .json_schema(&config.json_schema)
        )
    }
}
impl GenerateDocumentation<Extensions> for Extensions {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<Extensions>, DocumentationGenerationError> {
        Ok(TypedMarkdownDocumentation::new(self.clone()).extensions(&config.extensions))
    }
}
