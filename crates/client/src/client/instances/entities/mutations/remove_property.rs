#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::entities::variables::remove_property::variables::RemovePropertiesVariables;
    use crate::client::instances::entities::variables::remove_property::variables::RemovePropertiesVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use uuid::Uuid;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RemovePropertiesVariables")]
    pub struct RemoveProperties {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertiesVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemovePropertiesVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, removeProperties: $properties)]
        pub update: EntityInstance,
    }

    pub fn remove_property_by_name<S: Into<String>>(id: Uuid, property_name: S) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_by_name(id, property_name))
    }

    pub fn remove_property_by_type<P: Into<PropertyType>>(id: Uuid, property_type: P) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_by_type(id, property_type))
    }

    pub fn remove_properties_by_name(id: Uuid, property_names: Vec<String>) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_by_names(id, property_names))
    }

    pub fn remove_properties_by_type(id: Uuid, property_types: &PropertyTypes) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::new_by_types(id, property_types))
    }
}
