#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ComponentTypeId;
    use crate::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::entity_type::EntityTypeId;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct SearchEntityInstancesVariables {
        /// Filters the entity instances by type.
        #[builder(default)]
        pub ty: Option<EntityTypeId>,
        /// Returns only the entity instance with the given id.
        #[builder(default)]
        pub id: Option<UUID>,
        /// Returns the entity instance with the given label.
        #[builder(default)]
        pub label: Option<String>,
        // TODO: Filter by name.
        // TODO: Filter by description.
        /// Filter by properties.
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
        /// Filter by components.
        #[builder(default)]
        pub components: Option<Vec<ComponentTypeId>>,
        // TODO: search for behaviours
    }
}
