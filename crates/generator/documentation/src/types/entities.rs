use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::types::GenerateDocumentation;
use reactive_graph_graph::EntityType;

impl GenerateDocumentation<EntityType> for EntityType {
    fn generate_documentation(&self) -> Result<MarkdownDocumentation<EntityType>, DocumentationGenerationError> {
        Ok(MarkdownDocumentation::new(self.clone())
            .ty()
            .description()
            .components()
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
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypes;
    use std::fs::remove_file;

    #[test]
    pub fn test_generate_entity_type_documentation() {
        let entity_type = EntityType::default_test();
        let md = entity_type.generate_documentation().expect("Failed to generate documentation for entity type");
        println!("{md}");
    }

    #[test]
    pub fn test_generate_entity_types_documentations() {
        EntityTypes::default_test()
            .generate_documentations()
            .expect("Failed to generate documentations for entity types");
    }

    #[test]
    pub fn test_write_entity_type_documentation() {
        let entity_type = EntityType::default_test();
        let path = entity_type
            .generate_documentation()
            .expect("Failed to generate documentation for entity type")
            .write()
            .expect("Failed to write entity type documentation");
        remove_file(path).expect("Failed to delete entity type documentation");
    }

    #[test]
    pub fn test_write_entity_types_documentations() {
        EntityTypes::default_test()
            .write_documentations()
            .expect("Failed to write documentations for entity types");
    }
}
