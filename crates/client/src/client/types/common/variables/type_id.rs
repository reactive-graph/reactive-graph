#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    use reactive_graph_graph::NamespacedTypeGetter;

    impl<TY> From<TY> for TypeIdVariables
    where
        TY: NamespacedTypeGetter,
    {
        fn from(ty: TY) -> Self {
            TypeIdVariables {
                _type: ty.namespace().to_string(),
            }
        }
    }

    // TODO: DSD Rename TypeIdVariables to NamespacedTypeVariables
    #[derive(QueryVariables, Debug)]
    pub struct TypeIdVariables {
        _type: String,
    }
}
