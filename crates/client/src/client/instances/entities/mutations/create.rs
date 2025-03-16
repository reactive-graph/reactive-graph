#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::PropertyInstanceDefinition;
    use crate::PropertyInstanceDefinitions;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use typed_builder::TypedBuilder;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateEntityInstanceVariables {
        pub namespace: String,
        pub type_name: String,
        pub id: Option<UUID>,
        #[builder(default)]
        pub description: Option<String>,
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateEntityInstanceVariables")]
    pub struct CreateEntityInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityInstanceVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateEntityInstanceVariables")]
    pub struct MutationEntityInstances {
        #[arguments(type: { namespace: $namespace, name: $type_name}, id: $id , description: $description, properties: $properties
        )]
        pub create: EntityInstance,
    }

    pub fn create(
        ty: EntityTypeId,
        id: Option<Uuid>,
        description: Option<String>,
        properties: reactive_graph_graph::PropertyInstances,
    ) -> Operation<CreateEntityInstance, CreateEntityInstanceVariables> {
        use cynic::MutationBuilder;
        let id = id.map(|id| id.into());
        let properties: PropertyInstanceDefinitions = properties.into();
        let properties = Some(properties.0);
        let vars = CreateEntityInstanceVariables::builder()
            .namespace(ty.namespace())
            .type_name(ty.type_name())
            .id(id)
            .description(description)
            .properties(properties)
            .build();
        CreateEntityInstance::build(vars)
    }
}
