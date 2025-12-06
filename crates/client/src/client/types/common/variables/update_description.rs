#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct UpdateDescriptionVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub description: String,
    }

    impl UpdateDescriptionVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, description: String) -> Self {
            UpdateDescriptionVariables {
                _type: ty.namespace().to_string(),
                description,
            }
        }
    }
}
