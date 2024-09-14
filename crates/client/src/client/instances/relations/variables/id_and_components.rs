#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::id::UUID;
    use crate::schema_graphql::types::component::ComponentTypeId;
    use crate::schema_graphql::types::component::ComponentTypeIds;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(QueryVariables, Debug)]
    pub struct RelationInstanceIdAndComponentsVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The relation type id namespace.
        pub namespace: String,
        /// The relation type id type name.
        pub name: String,
        /// The relation type id type name.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
        // List of components.
        pub components: Option<Vec<ComponentTypeId>>,
    }

    impl RelationInstanceIdAndComponentsVariables {
        pub fn new<TY: Into<reactive_graph_graph::ComponentTypeIds>>(id: &RelationInstanceId, component_tys: TY) -> Self {
            let ty = id.ty.relation_type_id();
            let component_tys = component_tys.into();
            let component_tys: ComponentTypeIds = component_tys.into();
            Self {
                outbound_id: id.outbound_id.into(),
                namespace: ty.namespace(),
                name: ty.type_name(),
                instance_id: id.ty.instance_id(),
                inbound_id: id.inbound_id.into(),
                components: Some(component_tys.0),
            }
        }

        pub fn new_from_component_type<TY: Into<reactive_graph_graph::ComponentTypeId>>(id: &RelationInstanceId, component_ty: TY) -> Self {
            Self::new(id, reactive_graph_graph::ComponentTypeIds::new().component(component_ty))
        }
    }
}
