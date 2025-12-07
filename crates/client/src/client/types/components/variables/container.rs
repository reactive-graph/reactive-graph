#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    use reactive_graph_graph::ComponentContainerGetter;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct ComponentContainerVariables {
        pub _type: String,
        pub component_type: String,
    }

    impl<CTY> From<CTY> for ComponentContainerVariables
    where
        CTY: ComponentContainerGetter,
    {
        fn from(ty: CTY) -> Self {
            let container_ty = ty.container_ty();
            let component_ty = ty.component_ty();
            ComponentContainerVariables {
                _type: container_ty.namespace().to_string(),
                component_type: component_ty.namespace().to_string(),
            }
        }
    }
}
