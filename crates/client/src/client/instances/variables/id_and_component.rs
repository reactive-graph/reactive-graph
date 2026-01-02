#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::NamespacedTypeIdContainer;

    #[derive(QueryVariables, Debug)]
    pub struct IdAndComponentVariables {
        pub id: UUID,
        pub components: Option<Vec<String>>,
    }

    impl IdAndComponentVariables {
        pub fn new<ID: Into<UUID>, TYS: Into<ComponentTypeIds>>(id: ID, components: Option<TYS>) -> Self {
            Self {
                id: id.into(),
                components: components.map(|components| components.into().into_fully_qualified_namespaces()),
            }
        }
    }
}
