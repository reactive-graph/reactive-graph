#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    use crate::ExtensionDefinition;
    use reactive_graph_graph::Extension;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct AddExtensionVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub extension: ExtensionDefinition,
    }

    impl AddExtensionVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, extension: Extension) -> Self {
            Self {
                _type: ty.namespace().to_string(),
                extension: extension.into(),
            }
        }
    }
}
