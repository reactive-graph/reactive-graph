#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct PropertyContainerVariables {
        pub namespace: String,
        pub name: String,
        pub property_name: String,
    }

    impl PropertyContainerVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, property_name: String) -> Self {
            PropertyContainerVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
                property_name,
            }
        }
    }
}
