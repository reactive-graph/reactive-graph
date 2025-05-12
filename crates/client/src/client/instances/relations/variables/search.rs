#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct SearchRelationInstancesVariables {
        //
        // TODO: outboundComponentTy
        // TODO: outboundEntityTy
        // TODO: inboundComponentTy
        // TODO: inboundEntityTy
        // TODO (GraphQL Schema): search for instanceId
        //
        /// Returns only the relation instance with the outbound entity instance has the given id.
        #[builder(default)]
        pub outbound_id: Option<crate::schema_graphql::scalar::id::UUID>,
        /// Filters the relation instances by type.
        #[builder(default)]
        pub ty: Option<crate::schema_graphql::types::relation_type::RelationTypeId>,
        /// Returns only the relation instance with the inbound entity instance has the given id.
        #[builder(default)]
        pub inbound_id: Option<crate::schema_graphql::scalar::id::UUID>,
        /// Filter by properties.
        #[builder(default)]
        pub properties: Option<Vec<crate::schema_graphql::instances::property_instance::property_instance_definition::PropertyInstanceDefinition>>,
        /// Filter by components.
        #[builder(default)]
        pub components: Option<Vec<crate::schema_graphql::types::component::ComponentTypeId>>,
        // TODO: search for applied behaviours
    }
}
