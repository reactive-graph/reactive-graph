#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::client::instances::relations::variables::search::variables::SearchRelationInstancesVariables;
    use crate::client::instances::relations::variables::search::variables::SearchRelationInstancesVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchRelationInstancesVariables")]
    pub struct SearchRelationInstances {
        pub instances: SearchRelationInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "SearchRelationInstancesVariables")]
    pub struct SearchRelationInstancesInstances {
        #[arguments(outboundId: $outbound_id, type: $ty, inboundId: $inbound_id, properties: $properties, components: $components
        )]
        pub relations: Vec<RelationInstance>,
    }

    pub fn search(vars: SearchRelationInstancesVariables) -> Operation<SearchRelationInstances, SearchRelationInstancesVariables> {
        use cynic::QueryBuilder;
        SearchRelationInstances::build(vars)
    }
}
