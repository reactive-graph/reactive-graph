#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use reactive_graph_graph::ExtensionContainerGetter;
    use reactive_graph_graph::NamespacedTypeGetter;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ExtensionContainerVariables {
        pub namespace: String,
        pub name: String,
        pub extension_namespace: String,
        pub extension_name: String,
    }

    impl<ETY> From<ETY> for ExtensionContainerVariables
    where
        ETY: ExtensionContainerGetter,
    {
        fn from(ty: ETY) -> Self {
            let container_ty = ty.container_ty();
            let extension_ty = ty.extension_ty();
            Self::builder()
                .namespace(container_ty.namespace())
                .name(container_ty.type_name())
                .extension_namespace(extension_ty.namespace())
                .extension_name(extension_ty.type_name())
                .build()
        }
    }
}
