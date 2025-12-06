#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    use crate::schema_graphql::scalar::UUID;

    #[derive(QueryVariables, Debug)]
    pub struct SearchFlowInstancesVariables {
        /// Filters the fully qualified namespace of the entity type of the flow instance.
        #[cynic(rename = "type")]
        pub _type: Option<String>,
        /// Returns only the flow instance with the given id.
        pub id: Option<UUID>,
        /// Returns the flow instance with the given label.
        pub label: Option<String>,
        // TODO: Filter by name.
        // TODO: Filter by description.
        // TODO: Filter by entity instance.
        // TODO: Filter by relation instance.
    }
}
