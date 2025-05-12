#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::flows::variables::search::variables::SearchFlowInstancesVariables;
    use crate::client::instances::flows::variables::search::variables::SearchFlowInstancesVariablesFields;
    use crate::schema_graphql::instances::flow_instance::FlowInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchFlowInstancesVariables")]
    pub struct SearchFlowInstances {
        pub instances: SearchFlowInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "SearchFlowInstancesVariables")]
    pub struct SearchFlowInstancesInstances {
        #[arguments(type: $ty, id: $id, label: $label)]
        pub flows: Vec<FlowInstance>,
    }

    pub fn search(vars: SearchFlowInstancesVariables) -> Operation<SearchFlowInstances, SearchFlowInstancesVariables> {
        use cynic::QueryBuilder;
        SearchFlowInstances::build(vars)
    }
}
