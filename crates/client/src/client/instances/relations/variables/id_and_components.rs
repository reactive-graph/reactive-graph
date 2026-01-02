#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::id::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::ComponentTypeIds;
    use reactive_graph_graph::NamespacedTypeIdContainer;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(QueryVariables, Debug)]
    pub struct RelationInstanceIdAndComponentsVariables {
        /// The id of the outbound entity instance.
        pub outbound_id: UUID,
        /// The fully qualified namespace of the relation type.
        #[cynic(rename = "type")]
        pub _type: String,
        /// The relation instance id.
        pub instance_id: String,
        /// The id of the inbound entity instance.
        pub inbound_id: UUID,
        /// The list of components.
        pub components: Option<Vec<String>>,
    }

    impl RelationInstanceIdAndComponentsVariables {
        pub fn new<TY: Into<ComponentTypeIds>>(id: &RelationInstanceId, component_tys: TY) -> Self {
            let ty = id.ty.relation_type_id();
            let component_tys = component_tys.into().into_fully_qualified_namespaces();
            Self {
                outbound_id: id.outbound_id.into(),
                _type: ty.to_string(),
                instance_id: id.ty.instance_id().to_string(),
                inbound_id: id.inbound_id.into(),
                components: Some(component_tys),
            }
        }

        pub fn new_from_component_type<TY: Into<reactive_graph_graph::ComponentTypeId>>(id: &RelationInstanceId, component_ty: TY) -> Self {
            Self::new(id, reactive_graph_graph::ComponentTypeIds::new().component(component_ty))
        }
    }
}
