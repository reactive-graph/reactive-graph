#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationTypeId;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RelationTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<RelationTypeId> for RelationTypeIdVariables {
        fn from(ty: RelationTypeId) -> Self {
            RelationTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }
}
