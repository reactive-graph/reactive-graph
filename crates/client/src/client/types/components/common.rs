#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ComponentTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<ComponentTypeId> for ComponentTypeIdVariables {
        fn from(ty: ComponentTypeId) -> Self {
            ComponentTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }
}
