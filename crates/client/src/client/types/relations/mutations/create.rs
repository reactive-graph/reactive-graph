#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use crate::schema_graphql::types::relation_type::RelationType;
    use crate::types::relations::variables::create::variables::CreateRelationTypeVariables;
    use crate::types::relations::variables::create::variables::CreateRelationTypeVariablesFields;
    use reactive_graph_graph::InboundOutboundType;
    use reactive_graph_graph::MatchingInboundOutboundType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateRelationTypeVariables")]
    pub struct CreateRelationType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateRelationTypeVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateRelationTypeVariables")]
    pub struct MutationRelationTypes {
        #[arguments(outboundType: { entityType: $outbound_entity_type }, type: $_type, inboundType: { entityType: $inbound_entity_type }, description: $description, properties: $properties, extensions: $extensions
        )]
        pub create: RelationType,
    }

    pub fn create_relation_type_mutation(relation_type: reactive_graph_graph::RelationType) -> Operation<CreateRelationType, CreateRelationTypeVariables> {
        use cynic::MutationBuilder;
        let _type = relation_type.namespace().to_string();
        let description = relation_type.description;
        let property_types: PropertyTypeDefinitions = relation_type.properties.into();
        let extensions: ExtensionDefinitions = relation_type.extensions.into();
        let outbound_entity_type = match relation_type.outbound_type.clone() {
            InboundOutboundType::Component(_) => None,
            InboundOutboundType::EntityType(outbound_entity_ty) => match outbound_entity_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => Some(ty.namespace().to_string()),
                MatchingInboundOutboundType::Any => Some("*".to_string()),
            },
        };
        let outbound_component = match relation_type.outbound_type {
            InboundOutboundType::Component(outbound_component_ty) => match outbound_component_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => Some(ty.namespace().to_string()),
                MatchingInboundOutboundType::Any => Some("*".to_string()),
            },
            InboundOutboundType::EntityType(_) => None,
        };
        let inbound_entity_type = match relation_type.inbound_type.clone() {
            InboundOutboundType::Component(_) => None,
            InboundOutboundType::EntityType(inbound_entity_ty) => match inbound_entity_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => Some(ty.namespace().to_string()),
                MatchingInboundOutboundType::Any => Some("*".to_string()),
            },
        };
        let inbound_component = match relation_type.inbound_type {
            InboundOutboundType::Component(inbound_component_ty) => match inbound_component_ty {
                MatchingInboundOutboundType::NamespacedType(ty) => Some(ty.namespace().to_string()),
                MatchingInboundOutboundType::Any => Some("*".to_string()),
            },
            InboundOutboundType::EntityType(_) => None,
        };
        let vars = CreateRelationTypeVariables {
            outbound_entity_type,
            outbound_component,
            _type,
            inbound_entity_type,
            inbound_component,
            description: Some(description),
            properties: Some(property_types.0),
            extensions: Some(extensions.0),
        };
        CreateRelationType::build(vars)
    }
}
