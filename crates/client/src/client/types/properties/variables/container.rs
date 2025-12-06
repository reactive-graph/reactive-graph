#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct PropertyContainerVariables {
        /// The fully qualified namespace of the type.
        #[cynic(rename = "type")]
        pub _type: String,
        pub property_name: String,
    }

    impl PropertyContainerVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, property_name: String) -> Self {
            PropertyContainerVariables {
                _type: ty.namespace().to_string(),
                property_name,
            }
        }
    }
}
