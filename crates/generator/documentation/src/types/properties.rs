use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::TypeResolver;

impl GenerateDocumentation<PropertyType> for PropertyType {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<PropertyType>, DocumentationGenerationError> {
        Ok(TypedMarkdownDocumentation::new(self.clone()).property(&config.properties.property))
    }
}

impl GenerateDocumentation<PropertyTypes> for PropertyTypes {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<PropertyTypes>, DocumentationGenerationError> {
        Ok(TypedMarkdownDocumentation::new(self.clone()).own_properties(&config.properties))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DocumentationConfigPreset;
    use crate::FromDocumentationConfigPreset;
    use crate::GenerateDocumentation;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RandomNamespacedType;

    #[test]
    fn test_generate_property_type_documentation() {
        let property_type = PropertyType::random_type().unwrap();
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();
        let md = property_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for property type");
        println!("{md}");
    }
    #[test]
    fn test_generate_property_types_documentation() {
        let property_types = PropertyTypes::random_types_no_extensions();
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();
        let md = property_types
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for property types");
        println!("{md}");
    }
}
