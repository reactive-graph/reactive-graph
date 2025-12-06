#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {

    use crate::client::instances::entities::variables::add_property::variables::AddPropertiesVariables;
    use crate::client::instances::entities::variables::add_property::variables::AddPropertiesVariablesFields;
    use crate::schema_graphql::instances::entity_instance::EntityInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use uuid::Uuid;

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
        AddProperty::build(AddPropertiesVariables::new_property(id, property))
    }

    pub fn add_properties(id: Uuid, properties: PropertyTypes) -> Operation<AddProperty, AddPropertiesVariables> {
        use cynic::MutationBuilder;
        AddProperty::build(AddPropertiesVariables::new_properties(id, properties))
    }
}
