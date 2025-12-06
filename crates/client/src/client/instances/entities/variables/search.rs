#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;

    #[derive(QueryVariables, Debug)]
    pub struct SearchEntityInstancesVariables {
        /// Filters the entity instances by type.
        #[cynic(rename = "type")]
        pub _type: Option<String>,
        /// Returns only the entity instance with the given id.
        pub id: Option<UUID>,
        /// Returns the entity instance with the given label.
        pub label: Option<String>,
        // TODO: Filter by name.
        // TODO: Filter by description.
        /// Filter by properties.
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
        /// Filter the entity instances by components.
        pub components: Option<Vec<String>>,
        // TODO: search for behaviours
    }
}
