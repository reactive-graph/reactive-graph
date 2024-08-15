#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct EntityTypeIdVariables {
        namespace: String,
        name: String,
    }

    impl From<EntityTypeId> for EntityTypeIdVariables {
        fn from(ty: EntityTypeId) -> Self {
            EntityTypeIdVariables {
                namespace: ty.namespace(),
                name: ty.type_name(),
            }
        }
    }
}
