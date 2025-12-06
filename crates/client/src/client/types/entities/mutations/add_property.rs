#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariables;
    use crate::client::types::properties::variables::add_property::variables::AddPropertyVariablesFields;
    use crate::schema_graphql::types::entity_type::EntityType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::PropertyType;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddPropertyVariables")]
    pub struct AddProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertyVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: $_type, property: $property)]
        pub add_property: EntityType,
    }

    pub fn add_property_mutation<E: Into<EntityTypeId>, PT: Into<PropertyType>>(ty: E, property_type: PT) -> Operation<AddProperty, AddPropertyVariables> {
        use cynic::MutationBuilder;
        AddProperty::build(AddPropertyVariables::new(ty.into(), property_type.into()))
    }
}
