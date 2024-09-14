#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::variables::label::queries::LabelVariables;
    use crate::client::instances::variables::label::queries::LabelVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "LabelVariables")]
    pub struct GetEntityInstanceByLabel {
        pub instances: GetEntityInstanceByLabelInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "LabelVariables")]
    pub struct GetEntityInstanceByLabelInstances {
        #[arguments(label: $label)]
        pub entities: Vec<EntityInstance>,
    }

    pub fn get_entity_instance_by_label(label: String) -> Operation<GetEntityInstanceByLabel, LabelVariables> {
        use cynic::QueryBuilder;
        GetEntityInstanceByLabel::build(label.into())
    }
}
