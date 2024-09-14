#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::component::ComponentTypeId;
    use crate::schema_graphql::types::entity_type::EntityTypeId;
    use crate::PropertyInstanceDefinition;

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
        /// Filter by properties.
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
        /// Filter by components.
        #[builder(default)]
        pub components: Option<Vec<ComponentTypeId>>,
        // TODO: search for behaviours
    }

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
