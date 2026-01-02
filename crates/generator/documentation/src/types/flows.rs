use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::TypeResolver;

impl GenerateDocumentation<FlowType> for FlowType {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        _resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<FlowType>, DocumentationGenerationError> {
        Ok(TypedMarkdownDocumentation::new(self.clone())
            .ty(config)
            .description(config)
            .variables(&config.properties)
            .extensions(&config.extensions))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::DocumentationConfigPreset;
    use crate::FromDocumentationConfigPreset;
    use crate::GenerateDocumentations;
    use crate::generator::type_definition::MarkdownDocumentationWriter;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::Components;
    use reactive_graph_graph::EntityTypes;
    use reactive_graph_graph::FlowType;
    use reactive_graph_graph::FlowTypes;
    use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_graph::RandomNamespacedTypesWithId;
    use reactive_graph_graph::RelationTypes;
    use reactive_graph_graph::TypeSystem;
    use reactive_graph_utils_test::r_temp_dir;
    use std::fs::remove_dir_all;
    use std::fs::remove_file;

    #[test]
    fn test_generate_flow_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let flow_type = FlowType::random_type().unwrap();
        let flow_types = FlowTypes::new();
        flow_types.push(flow_type.clone());
        let relation_types = RelationTypes::random_types_with_ids(&flow_type.uses_relation_types()).unwrap();
        let relation_type_components = relation_types.get_component_type_ids();
        let entity_types = EntityTypes::random_types_with_ids(&flow_type.uses_entity_types()).unwrap();
        let entity_type_components = entity_types.get_component_type_ids();
        let mut components = ComponentTypeIds::new();
        components.extend(entity_type_components);
        components.extend(relation_type_components);
        let components = Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap();
        let resolver = TypeSystem::builder()
            .components(components)
            .entity_types(entity_types.clone())
            .relation_types(relation_types.clone())
            .flow_types(flow_types)
            .build()
            .into();

        let md = flow_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for flow type");
        println!("{md}");
    }

    #[test]
    fn test_generate_flow_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let flow_types = FlowTypes::random_types(0..10).unwrap();
        let resolver = TypeResolver::new();
        flow_types
            .generate_typed_documentations(&config, &resolver)
            .expect("Failed to generate documentations for flow types");
    }

    #[test]
    fn test_write_flow_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let flow_type = FlowType::random_type().unwrap();
        let resolver = TypeResolver::new();
        let path = flow_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for flow type")
            .write(&temp_dir)
            .expect("Failed to write flow type documentation");
        assert!(path.exists());
        remove_file(path).expect("Failed to delete flow type documentation");
    }

    #[test]
    fn test_write_flow_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let flow_types = FlowTypes::random_types(1..10).unwrap();
        let entity_type_ids = flow_types.iter().map(|flow_type| flow_type.uses_entity_types()).collect();
        let relation_type_ids = flow_types.iter().map(|flow_type| flow_type.uses_relation_types()).collect();
        let relation_types = RelationTypes::random_types_with_ids(&relation_type_ids).unwrap();
        let relation_type_components = relation_types.get_component_type_ids();
        let entity_types = EntityTypes::random_types_with_ids(&entity_type_ids).unwrap();
        let entity_type_components = entity_types.get_component_type_ids();
        let mut components = ComponentTypeIds::new();
        components.extend(entity_type_components);
        components.extend(relation_type_components);
        let components = Components::random_types_with_ids(&entity_types.get_component_type_ids()).unwrap();
        let resolver = TypeSystem::builder()
            .components(components)
            .entity_types(entity_types.clone())
            .relation_types(relation_types.clone())
            .flow_types(flow_types.clone())
            .build()
            .into();

        let paths = flow_types
            .write_documentations(&config, &resolver, &temp_dir)
            .expect("Failed to write documentations for flow types");
        for path in paths {
            assert!(path.exists());
        }
        remove_dir_all(temp_dir).expect("Failed to delete temp dir {temp_dir:?}");
    }
}
