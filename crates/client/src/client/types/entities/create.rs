#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::schema_graphql::types::extension::ExtensionDefinition;
    use crate::schema_graphql::types::extension::ExtensionDefinitions;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinitions;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateEntityTypeVariables {
        pub namespace: String,
        pub name: String,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateEntityTypeVariables")]
    pub struct CreateEntityType {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityTypeVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityTypeVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, description: $description, properties: $properties, extensions: $extensions
        )]
        pub create: EntityType,
    }

    pub fn create_entity_type_mutation(entity_type: reactive_graph_graph::EntityType) -> Operation<CreateEntityType, CreateEntityTypeVariables> {
        use cynic::MutationBuilder;
        let namespace = entity_type.namespace();
        let name = entity_type.type_name();
        let description = entity_type.description;
        let property_types: PropertyTypeDefinitions = entity_type.properties.into();
        let extensions: ExtensionDefinitions = entity_type.extensions.into();
        let vars = CreateEntityTypeVariables {
            namespace,
            name,
            description: Some(description),
            properties: Some(property_types.0),
            extensions: Some(extensions.0),
        };
        CreateEntityType::build(vars)
    }

    pub fn create_entity_type_with_variables(variables: CreateEntityTypeVariables) -> Operation<CreateEntityType, CreateEntityTypeVariables> {
        use cynic::MutationBuilder;
        CreateEntityType::build(variables)
    }
}
