#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::entities::variables::search::variables::SearchEntityInstancesVariables;
    use crate::client::instances::entities::variables::search::variables::SearchEntityInstancesVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchEntityInstancesVariables")]
    pub struct SearchEntityInstances {
        pub instances: SearchEntityInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "SearchEntityInstancesVariables")]
    pub struct SearchEntityInstancesInstances {
        #[arguments(type: $ty, id: $id, label: $label, properties: $properties, components: $components
        )]
        pub entities: Vec<EntityInstance>,
    }

    pub fn search(vars: SearchEntityInstancesVariables) -> Operation<SearchEntityInstances, SearchEntityInstancesVariables> {
        use cynic::QueryBuilder;
        SearchEntityInstances::build(vars)
    }
}
