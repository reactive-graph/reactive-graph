#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
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
        /// Query by properties.
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
        // TODO: search for behaviours
        // TODO: search for components
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchEntityInstancesVariables")]
    pub struct SearchEntityInstances {
        pub instances: SearchEntityInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "SearchEntityInstancesVariables")]
    pub struct SearchEntityInstancesInstances {
        #[arguments(type: $ty, id: $id, label: $label, properties: $properties
        )]
        pub entities: Vec<EntityInstance>,
    }

    pub fn search(vars: SearchEntityInstancesVariables) -> Operation<SearchEntityInstances, SearchEntityInstancesVariables> {
        use cynic::QueryBuilder;
        // let ty = reactive_graph_graph::EntityTypeId::new_from_type("test", "test_toml");
        // let vars = SearchEntityInstanceVariables::builder().ty(Some(ty.into())).build();
        SearchEntityInstances::build(vars)
    }
}
