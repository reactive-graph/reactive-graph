#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::create::variables::CreateRelationInstanceVariables;
    use crate::client::instances::relations::variables::create::variables::CreateRelationInstanceVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::PropertyInstances;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "CreateRelationInstanceVariables")]
    pub struct CreateRelationInstance {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateRelationInstanceVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "CreateRelationInstanceVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, namespace: $namespace, typeName: $type_name, instanceId: $instance_id, inboundId: $inbound_id}, description: $description, properties: $properties
        )]
        pub create: RelationInstance,
    }

    pub fn create(
        id: &RelationInstanceId,
        description: Option<String>,
        properties: PropertyInstances,
    ) -> Operation<CreateRelationInstance, CreateRelationInstanceVariables> {
        use cynic::MutationBuilder;
        CreateRelationInstance::build(CreateRelationInstanceVariables::new(id, description, properties))
    }
}
