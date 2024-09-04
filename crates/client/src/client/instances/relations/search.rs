#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::component::ComponentTypeId;
    use crate::schema_graphql::types::relation_type::RelationTypeId;
    use crate::PropertyInstanceDefinition;

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
        pub outbound_id: Option<UUID>,
        /// Filters the relation instances by type.
        #[builder(default)]
        pub ty: Option<RelationTypeId>,
        /// Returns only the relation instance with the inbound entity instance has the given id.
        #[builder(default)]
        pub inbound_id: Option<UUID>,
        /// Filter by properties.
        #[builder(default)]
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
        /// Filter by components.
        #[builder(default)]
        pub components: Option<Vec<ComponentTypeId>>,
        // TODO: search for applied behaviours
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Query", variables = "SearchRelationInstancesVariables")]
    pub struct SearchRelationInstances {
        pub instances: SearchRelationInstancesInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(graphql_type = "Instances", variables = "SearchRelationInstancesVariables")]
    pub struct SearchRelationInstancesInstances {
        #[arguments(outboundId: $outbound_id, type: $ty, inboundId: $inbound_id, properties: $properties, components: $components
        )]
        pub relations: Vec<RelationInstance>,
    }

    pub fn search(vars: SearchRelationInstancesVariables) -> Operation<SearchRelationInstances, SearchRelationInstancesVariables> {
        use cynic::QueryBuilder;
        SearchRelationInstances::build(vars)
    }
}
