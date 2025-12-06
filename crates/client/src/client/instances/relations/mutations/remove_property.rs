#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::remove_property::variables::RemovePropertiesVariables;
    use crate::client::instances::relations::variables::remove_property::variables::RemovePropertiesVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RemovePropertiesVariables")]
    pub struct RemoveProperties {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertiesVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertiesVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, type: $_type, instanceId: $instance_id, inboundId: $inbound_id}, removeProperties: $properties)]
        pub update: RelationInstance,
    }

    pub fn remove_properties(id: &RelationInstanceId, property_names: Vec<String>) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new(id, property_names))
    }

    pub fn remove_property<S: Into<String>>(id: &RelationInstanceId, property_name: S) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_from_property_name(id, property_name.into()))
    }

    pub fn remove_properties_by_property_types<P: Into<PropertyTypes>>(
        id: &RelationInstanceId,
        property_types: P,
    ) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_from_property_types(id, property_types))
    }
    pub fn remove_property_by_property_type<P: Into<PropertyType>>(
        id: &RelationInstanceId,
        property_type: P,
    ) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_from_property_type(id, property_type))
    }
}
