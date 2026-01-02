#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::set_property::variables::SetPropertyVariables;
    use crate::client::instances::relations::variables::set_property::variables::SetPropertyVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationInstanceId;
    use serde_json::Value;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "SetPropertyVariables")]
    pub struct SetProperty {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, type: $_type, instanceId: $instance_id, inboundId: $inbound_id }, properties: $properties)]
        pub update: RelationInstance,
    }

    pub fn set_property(id: &RelationInstanceId, name: String, value: Value) -> Operation<SetProperty, SetPropertyVariables> {
        use cynic::MutationBuilder;
        SetProperty::build(SetPropertyVariables::new(id, name, value))
    }
}
