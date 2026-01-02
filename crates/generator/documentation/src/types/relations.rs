use crate::error::DocumentationGenerationError;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::GenerateDocumentation;
use crate::types::config::DocumentationConfig;
use crate::types::config::DocumentationConfigPreset;
use crate::types::config::FromDocumentationConfigPreset;
use convert_case::Case;
use convert_case::Casing;
use reactive_graph_graph::InboundOutboundDirection;
use reactive_graph_graph::InboundOutboundType;
use reactive_graph_graph::MatchingInboundOutboundType;
use reactive_graph_graph::NamespacedTypeConstructor;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;
use std::fmt::Display;

impl TypedMarkdownDocumentation<RelationType> {
    pub fn outbound_type(self) -> Self {
        let outbound_type = self.ty.outbound_type.clone();
        Self::inbound_outbound_type(self, outbound_type, InboundOutboundDirection::Outbound)
    }

    pub fn inbound_type(self) -> Self {
        let inbound_type = self.ty.inbound_type.clone();
        Self::inbound_outbound_type(self, inbound_type, InboundOutboundDirection::Inbound)
    }

    fn inbound_outbound_type(self, inbound_outbound_type: InboundOutboundType, dir: InboundOutboundDirection) -> Self {
        {
            let mut document = self.document.write().unwrap();
            match inbound_outbound_type {
                InboundOutboundType::Component(ty) => {
                    document.header2(format!("{} Component", dir.to_string().to_case(Case::UpperCamel)));
                    document.paragraph(matching_inbound_outbound_type(&ty));
                }
                InboundOutboundType::EntityType(ty) => {
                    document.header2(format!("{} Entity", dir.to_string().to_case(Case::UpperCamel)));
                    document.paragraph(matching_inbound_outbound_type(&ty));
                }
            };
        }
        self
    }
}

fn matching_inbound_outbound_type<T>(ty: &MatchingInboundOutboundType<T>) -> String
where
    T: NamespacedTypeGetter + TypeDefinitionGetter + NamespacedTypeConstructor + Clone + Display,
{
    match ty {
        MatchingInboundOutboundType::NamespacedType(ty) => {
            format!("`{}`", ty.namespace().to_string())
        }
        MatchingInboundOutboundType::Any => "`*`".to_string(),
    }
}

impl GenerateDocumentation<RelationType> for RelationType {
    fn generate_documentation(
        &self,
        config: &DocumentationConfig,
        resolver: &TypeResolver,
    ) -> Result<TypedMarkdownDocumentation<RelationType>, DocumentationGenerationError> {
        let config_components = match config.components.clone() {
            None => DocumentationConfig::new_from_preset(DocumentationConfigPreset::None),
            Some(config) => *config,
        };
        Ok(TypedMarkdownDocumentation::new(self.clone())
            .ty(config)
            .description(config)
            .outbound_type()
            .inbound_type()
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
    use reactive_graph_graph::NamespacedTypeComponentTypeIdContainer;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypes;
    use reactive_graph_graph::RandomNamespacedTypesWithId;
    use reactive_graph_graph::RelationType;
    use reactive_graph_graph::RelationTypes;
    use reactive_graph_graph::TypeSystem;
    use reactive_graph_utils_test::r_temp_dir;
    use std::fs::remove_dir_all;
    use std::fs::remove_file;

    #[test]
    fn test_generate_relation_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let relation_type = RelationType::random_type().unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&relation_type.components).unwrap())
            .relation_types(RelationTypes::new().relation(relation_type.clone()))
            .build();
        let resolver = TypeResolver::from(type_system);
        let md = relation_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for relation type");
        println!("{md}");
    }

    #[test]
    fn test_generate_relation_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);
        let relation_types = RelationTypes::random_types(0..10).unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&relation_types.get_component_type_ids()).unwrap())
            .relation_types(relation_types.clone())
            .build();
        let resolver = TypeResolver::from(type_system);
        relation_types
            .generate_typed_documentations(&config, &resolver)
            .expect("Failed to generate documentations for relation types");
    }

    #[test]
    fn test_write_relation_type_documentation() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);

        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let relation_type = RelationType::random_type().unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&relation_type.components).unwrap())
            .relation_types(RelationTypes::new().relation(relation_type.clone()))
            .build();
        let resolver = TypeResolver::from(type_system);
        let path = relation_type
            .generate_documentation(&config, &resolver)
            .expect("Failed to generate documentation for relation type")
            .write(&temp_dir)
            .expect("Failed to write relation type documentation");
        remove_file(path).expect("Failed to delete relation type documentation");
    }

    #[test]
    fn test_write_relation_types_documentations() {
        let config = DocumentationConfig::new_from_preset(DocumentationConfigPreset::Full);

        let temp_dir = r_temp_dir();
        println!("temp dir path: {temp_dir:?}");

        let relation_types = RelationTypes::random_types(1..10).unwrap();
        let type_system = TypeSystem::builder()
            .components(Components::random_types_with_ids(&relation_types.get_component_type_ids()).unwrap())
            .relation_types(relation_types.clone())
            .build();
        let resolver = TypeResolver::from(type_system);

        let paths = relation_types
            .write_documentations(&config, &resolver, &temp_dir)
            .expect("Failed to write documentations for relation types");
        for path in paths {
            assert!(path.exists());
        }
        remove_dir_all(temp_dir).expect("Failed to delete temp dir {temp_dir:?}");
    }
}
