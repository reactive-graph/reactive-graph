#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use typed_builder::TypedBuilder;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RemovePropertiesVariables {
        pub id: UUID,
        pub properties: Option<Vec<String>>,
    }

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

    pub fn remove_property<S: Into<String>>(id: Uuid, property_name: S) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        let property_name = property_name.into();
        RemoveProperties::build(RemovePropertiesVariables::builder().id(id.into()).properties(Some(vec![property_name])).build())
    }

    pub fn remove_property_by_type<P: Into<PropertyType>>(id: Uuid, property_type: P) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        let property_type = property_type.into();
        RemoveProperties::build(
            RemovePropertiesVariables::builder()
                .id(id.into())
                .properties(Some(vec![property_type.name]))
                .build(),
        )
    }

    pub fn remove_properties(id: Uuid, properties: Vec<String>) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        RemoveProperties::build(RemovePropertiesVariables::builder().id(id.into()).properties(Some(properties)).build())
    }

    pub fn remove_properties_by_type(id: Uuid, properties: &PropertyTypes) -> Operation<RemoveProperties, RemovePropertiesVariables> {
        use cynic::MutationBuilder;
        let properties = properties.iter().map(|property_type| property_type.name.clone()).collect();
        RemoveProperties::build(RemovePropertiesVariables::builder().id(id.into()).properties(Some(properties)).build())
    }
}
