#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct TypeIdVariables {
        namespace: String,
        name: String,
    }

    impl<TY> From<TY> for TypeIdVariables
    where
        TY: NamespacedTypeGetter,
    {
        fn from(ty: TY) -> Self {
            TypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }
}
