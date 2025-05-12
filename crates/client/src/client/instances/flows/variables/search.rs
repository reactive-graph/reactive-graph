#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::entity_type::EntityTypeId;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct SearchFlowInstancesVariables {
        /// Filters the flow instances by type.
        #[builder(default)]
        pub ty: Option<EntityTypeId>,
        /// Returns only the flow instance with the given id.
        #[builder(default)]
        pub id: Option<UUID>,
        /// Returns the flow instance with the given label.
        #[builder(default)]
        pub label: Option<String>,
        // TODO: Filter by name.
        // TODO: Filter by description.
        // TODO: Filter by entity instance.
        // TODO: Filter by relation instance.
    }
}
