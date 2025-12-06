use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::TypeResolver;

impl GenerateDocumentation<Component> for Component {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<Component>, DocumentationGenerationError> {
        Ok(TypedMarkdownDocumentation::new(self.clone())
            .ty(config)
            .description(config)
            .own_properties(&config.properties)
            .extensions(&config.extensions)
            .json_schema(&config.json_schema))
    }
}

impl GenerateDocumentation<ComponentTypeIds> for ComponentTypeIds {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<ComponentTypeIds>, DocumentationGenerationError> {
        TypedMarkdownDocumentation::new(self.clone()).components(config, resolver)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DocumentationConfigPreset;
    use crate::FromDocumentationConfigPreset;
    use crate::GenerateDocumentations;
    use crate::generator::type_definition::MarkdownDocumentationWriter;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_utils_test::r_temp_dir;
    use std::fs::remove_dir_all;
    use std::fs::remove_file;

    #[test]
    fn test_generate_component_documentation() {
        let component = Component::random_type().unwrap();
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();
        let md = component
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for component");
        println!("{md}");
    }

    #[test]
    fn test_generate_components_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();
        Components::random_types(0..10)
            .unwrap()
            .generate_typed_documentations(&config, &resolver)
            .expect("Failed to generate documentations for components");
    }

    #[test]
    fn test_write_component_documentation() {
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let component = Component::random_type().unwrap();
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();

        let path = component
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for component")
            .write(&temp_dir)
            .expect("Failed to write component documentation");
        assert!(path.exists());
        remove_file(path).expect("Failed to delete component documentation");
    }

    #[test]
    fn test_write_components_documentations() {
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let resolver = TypeResolver::new();

        let paths = Components::random_types(1..10)
            .unwrap()
            .write_documentations(&config, &resolver, &temp_dir)
            .expect("Failed to write documentations for components");
        for path in paths {
            assert!(path.exists());
        }
        remove_dir_all(temp_dir).expect("Failed to delete temp dir {temp_dir:?}");
    }
}
