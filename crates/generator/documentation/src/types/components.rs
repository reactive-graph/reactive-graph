use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::types::GenerateDocumentation;
use reactive_graph_graph::Component;

impl GenerateDocumentation<Component> for Component {
    fn generate_documentation(&self) -> Result<MarkdownDocumentation<Component>, DocumentationGenerationError> {
        Ok(MarkdownDocumentation::new(self.clone())
            .ty()
            .description()
            .properties()
            .extensions()
            .json_schema())
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::GenerateDocumentations;
    use default_test::DefaultTest;
    use reactive_graph_graph::Component;
    use reactive_graph_graph::Components;
    use std::fs::remove_file;

    #[test]
    pub fn test_generate_component_documentation() {
        let component = Component::default_test();
        let md = component.generate_documentation().expect("Failed to generate documentation for component");
        println!("{md}");
    }

    #[test]
    pub fn test_generate_components_documentations() {
        Components::default_test()
            .generate_documentations()
            .expect("Failed to generate documentations for components");
    }

    #[test]
    pub fn test_write_component_documentation() {
        let component = Component::default_test();
        let path = component
            .generate_documentation()
            .expect("Failed to generate documentation for component")
            .write()
            .expect("Failed to write component documentation");
        remove_file(path).expect("Failed to delete component documentation");
    }

    #[test]
    pub fn test_write_components_documentations() {
        Components::default_test()
            .write_documentations()
            .expect("Failed to write documentations for components");
    }
}
