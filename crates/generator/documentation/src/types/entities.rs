use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use crate::types::config::DocumentationConfigPreset;
use crate::types::config::FromDocumentationConfigPreset;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::TypeResolver;

impl GenerateDocumentation<EntityType> for EntityType {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<EntityType>, DocumentationGenerationError> {
        let config_components = match config.components.clone() {
            None => DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short),
            Some(config) => *config,
        };
        Ok(TypedMarkdownDocumentation::new(self.clone())
            .ty(config)
            .description(config)
            .components(&config_components, resolver)?
            .own_properties(&config.properties)
            .component_properties(&config.properties, resolver, false)?
            .extensions(&config.extensions)
            .json_schema_with_components(&config.json_schema, resolver)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::GenerateDocumentations;
    use crate::generator::type_definition::MarkdownDocumentationWriter;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::EntityTypes;
    use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_graph::RandomNamespacedTypesWithId;
    use reactive_graph_graph::TypeSystem;
    use reactive_graph_utils_test::r_temp_dir;
    use std::fs::remove_dir_all;
    use std::fs::remove_file;

    #[test]
    fn test_generate_entity_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let entity_type = EntityType::random_type().unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_type.components).unwrap())
            .entity_types(EntityTypes::new().entity(entity_type.clone()))
            .build();
        let resolver = TypeResolver::from(type_system);

        let md = entity_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for entity type");
        println!("{md}");
    }

    #[test]
    fn test_generate_entity_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let entity_types = EntityTypes::random_types(0..10).unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap())
            .entity_types(entity_types.clone())
            .build();
        let resolver = TypeResolver::from(type_system);
        entity_types
            .generate_typed_documentations(&config, &resolver)
            .expect("Failed to generate documentations for entity types");
    }

    #[test]
    fn test_write_entity_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let entity_type = EntityType::random_type().unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_type.components).unwrap())
            .entity_types(EntityTypes::new().entity(entity_type.clone()))
            .build();
        let resolver = TypeResolver::from(type_system);
        let path = entity_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for entity type")
            .write(&temp_dir)
            .expect("Failed to write entity type documentation");
        remove_file(path).expect("Failed to delete entity type documentation");
    }

    #[test]
    fn test_write_entity_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let entity_types = EntityTypes::random_types(1..10).unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap())
            .entity_types(entity_types.clone())
            .build();
        let resolver = TypeResolver::from(type_system);

        let paths = entity_types
            .write_documentations(&config, &resolver, &temp_dir)
            .expect("Failed to write documentations for entity types");
        for path in paths {
            assert!(path.exists());
        }
        remove_dir_all(temp_dir).expect("Failed to delete temp dir {temp_dir:?}");
    }
}
