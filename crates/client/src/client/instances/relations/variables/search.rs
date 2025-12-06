#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use cynic::QueryVariables;

    #[derive(QueryVariables, Debug)]
    pub struct SearchRelationInstancesVariables {
        //
        // TODO: outboundComponentTy
        // TODO: outboundEntityTy
        // TODO: inboundComponentTy
        // TODO: inboundEntityTy
        // TODO (GraphQL Schema): search for instanceId
        //
        /// Returns only the relation instance with the outbound entity instance has the given id.
        pub outbound_id: Option<crate::schema_graphql::scalar::id::UUID>,
        /// Filters the relation instances by fully qualified namespace of the relation type.
        #[cynic(rename = "type")]
        pub _type: Option<String>,
        /// Returns only the relation instance with the inbound entity instance has the given id.
        pub inbound_id: Option<crate::schema_graphql::scalar::id::UUID>,
        /// Filter by properties.
        pub properties: Option<Vec<crate::schema_graphql::instances::property_instance::property_instance_definition::PropertyInstanceDefinition>>,
        /// Filter the relation instances by components.
        pub components: Option<Vec<String>>,
        // TODO: search for applied behaviours
    }
}
