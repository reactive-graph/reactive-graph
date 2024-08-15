#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use reactive_graph_graph::EntityComponentTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationComponentTypeId;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct ComponentContainerVariables {
        pub namespace: String,
        pub name: String,
        pub component_namespace: String,
        pub component_name: String,
    }

    impl From<EntityComponentTypeId> for ComponentContainerVariables {
        fn from(ty: EntityComponentTypeId) -> Self {
            Self::builder()
                .namespace(ty.entity_ty.namespace())
                .name(ty.entity_ty.type_name())
                .component_namespace(ty.component_ty.namespace())
                .component_name(ty.component_ty.type_name())
                .build()
        }
    }

    impl From<RelationComponentTypeId> for ComponentContainerVariables {
        fn from(ty: RelationComponentTypeId) -> Self {
            Self::builder()
                .namespace(ty.relation_ty.namespace())
                .name(ty.relation_ty.type_name())
                .component_namespace(ty.component_ty.namespace())
                .component_name(ty.component_ty.type_name())
                .build()
        }
    }
}
