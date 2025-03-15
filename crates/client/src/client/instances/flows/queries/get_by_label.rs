#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::variables::label::queries::LabelVariables;
    use crate::client::instances::variables::label::queries::LabelVariablesFields;
    use crate::schema_graphql::instances::flow_instance::FlowInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "LabelVariables")]
    pub struct GetFlowInstanceByLabel {
        pub instances: GetFlowInstanceByLabelInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "LabelVariables")]
    pub struct GetFlowInstanceByLabelInstances {
        #[arguments(label: $label)]
        pub flows: Vec<FlowInstance>,
    }

    pub fn get_flow_instance_by_label(label: String) -> Operation<GetFlowInstanceByLabel, LabelVariables> {
        use cynic::QueryBuilder;
        GetFlowInstanceByLabel::build(label.into())
    }
}
