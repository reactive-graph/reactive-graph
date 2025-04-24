#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct UpdateDescriptionVariables {
        pub namespace: String,
        pub name: String,
        pub description: String,
    }

    impl UpdateDescriptionVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, description: String) -> Self {
            UpdateDescriptionVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
                description,
            }
        }
    }
}
