#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use crate::schema_graphql::scalar::UUID;
    use crate::schema_graphql::types::property_type::PropertyTypeDefinition;
    use crate::PropertyTypeDefinitions;
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use typed_builder::TypedBuilder;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct AddPropertiesVariables {
        pub id: UUID,
        pub properties: Option<Vec<PropertyTypeDefinition>>,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddPropertiesVariables")]
    pub struct AddProperty {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertiesVariables")]
    pub struct MutationInstances {
        pub entities: MutationEntityInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertiesVariables")]
    pub struct MutationEntityInstances {
        #[arguments(id: $id, addProperties: $properties)]
        pub update: EntityInstance,
    }

    pub fn add_property(id: Uuid, property: PropertyType) -> Operation<AddProperty, AddPropertiesVariables> {
        use cynic::MutationBuilder;
        let property = property.into();
        let vars = AddPropertiesVariables::builder().id(id.into()).properties(Some(vec![property])).build();
        AddProperty::build(vars.into())
    }

    pub fn add_properties(id: Uuid, properties: PropertyTypes) -> Operation<AddProperty, AddPropertiesVariables> {
        use cynic::MutationBuilder;
        let properties: PropertyTypeDefinitions = properties.into();
        let vars = AddPropertiesVariables::builder().id(id.into()).properties(Some(properties.0)).build();
        AddProperty::build(vars.into())
    }
}
