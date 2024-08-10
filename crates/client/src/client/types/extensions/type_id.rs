#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::ExtensionTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ExtensionTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<ExtensionTypeId> for ExtensionTypeIdVariables {
        fn from(ty: ExtensionTypeId) -> Self {
            ExtensionTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }
}
