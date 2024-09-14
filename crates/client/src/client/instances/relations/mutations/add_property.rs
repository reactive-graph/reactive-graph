#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::instances::relations::variables::add_property::variables::AddPropertiesVariables;
    use crate::client::instances::relations::variables::add_property::variables::AddPropertiesVariablesFields;
    use crate::schema_graphql::instances::relation_instance::RelationInstance;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use reactive_graph_graph::RelationInstanceId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddPropertiesVariables")]
    pub struct AddProperty {
        pub instances: MutationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertiesVariables")]
    pub struct MutationInstances {
        pub relations: MutationRelationInstances,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddPropertiesVariables")]
    pub struct MutationRelationInstances {
        #[arguments(relationInstanceId: { outboundId: $outbound_id, namespace: $namespace, typeName: $name, instanceId: $instance_id, inboundId: $inbound_id}, addProperties: $properties)]
        pub update: RelationInstance,
    }

    pub fn add_property(id: &RelationInstanceId, property_type: PropertyType) -> Operation<AddProperty, AddPropertiesVariables> {
        use cynic::MutationBuilder;
        let vars = AddPropertiesVariables::new(id, PropertyTypes::new().property(property_type));
        AddProperty::build(vars.into())
    }

    pub fn add_properties(id: &RelationInstanceId, property_types: PropertyTypes) -> Operation<AddProperty, AddPropertiesVariables> {
        use cynic::MutationBuilder;
        let vars = AddPropertiesVariables::new(id, property_types);
        AddProperty::build(vars.into())
    }
}
