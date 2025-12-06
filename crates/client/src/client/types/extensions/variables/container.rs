#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::ExtensionContainerGetter;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct ExtensionContainerVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub extension_type: String,
    }

    impl<ETY> From<ETY> for ExtensionContainerVariables
    where
        ETY: ExtensionContainerGetter,
    {
        fn from(ty: ETY) -> Self {
            Self {
                _type: ty.container_ty().namespace().to_string(),
                extension_type: ty.extension_ty().namespace().to_string(),
            }
        }
    }
}
