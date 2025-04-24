#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::ComponentContainerGetter;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ComponentContainerVariables {
        pub namespace: String,
        pub name: String,
        pub component_namespace: String,
        pub component_name: String,
    }

    impl<CTY> From<CTY> for ComponentContainerVariables
    where
        CTY: ComponentContainerGetter,
    {
        fn from(ty: CTY) -> Self {
            let container_ty = ty.container_ty();
            let component_ty = ty.component_ty();
            Self::builder()
                .namespace(container_ty.namespace())
                .name(container_ty.type_name())
                .component_namespace(component_ty.namespace())
                .component_name(component_ty.type_name())
                .build()
        }
    }
}
