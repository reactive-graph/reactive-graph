#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::entities::variables::set_property::variables::SetPropertyVariables;
    use crate::client::instances::entities::variables::set_property::variables::SetPropertyVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use serde_json::Value;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "SetPropertyVariables")]
    pub struct SetProperty {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "SetPropertyVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, properties: $properties)]
        pub update: EntityInstance,
    }

    pub fn set_property(id: Uuid, name: String, value: Value) -> Operation<SetProperty, SetPropertyVariables> {
        use cynic::MutationBuilder;
        SetProperty::build(SetPropertyVariables::new(id, name, value))
    }
}
