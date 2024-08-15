#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use crate::schema_graphql::types::relation_type::RelationType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateRelationTypeVariables {
        pub outbound_type_namespace: String,
        pub outbound_type_name: String,
        pub namespace: String,
        pub name: String,
        pub inbound_type_namespace: String,
        pub inbound_type_name: String,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

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
        #[arguments(outboundType: { entityType: { name: $outbound_type_name, namespace: $outbound_type_namespace } }, type: { name: $name, namespace: $namespace }, inboundType: { entityType: { name: $inbound_type_name, namespace: $inbound_type_namespace } }, description: $description, properties: $properties, extensions: $extensions
        )]
        pub create: RelationType,
    }

    pub fn create_relation_type_mutation(relation_type: reactive_graph_graph::RelationType) -> Operation<CreateRelationType, CreateRelationTypeVariables> {
        use cynic::MutationBuilder;
        let namespace = relation_type.namespace();
        let name = relation_type.type_name();
        let description = relation_type.description;
        let property_types: PropertyTypeDefinitions = relation_type.properties.into();
        let extensions: ExtensionDefinitions = relation_type.extensions.into();
        let vars = CreateRelationTypeVariables {
            outbound_type_namespace: relation_type.outbound_type.namespace(),
            outbound_type_name: relation_type.outbound_type.type_name(),
            namespace,
            name,
            inbound_type_namespace: relation_type.inbound_type.namespace(),
            inbound_type_name: relation_type.inbound_type.type_name(),
            description: Some(description),
            properties: Some(property_types.0),
            extensions: Some(extensions.0),
        };
        CreateRelationType::build(vars)
    }

    pub fn create_relation_type_with_variables(variables: CreateRelationTypeVariables) -> Operation<CreateRelationType, CreateRelationTypeVariables> {
        use cynic::MutationBuilder;
        CreateRelationType::build(variables)
    }
}
