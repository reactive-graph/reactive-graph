#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::entities::variables::create::variables::CreateEntityInstanceVariables;
    use crate::client::instances::entities::variables::create::variables::CreateEntityInstanceVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityTypeId;
    use uuid::Uuid;

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
        #[arguments(type: $_type, id: $id, description: $description, properties: $properties
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
        CreateEntityInstance::build(CreateEntityInstanceVariables::new(ty, id, description, properties))
    }
}
