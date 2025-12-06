#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariables;
    use crate::client::types::properties::variables::container::variables::PropertyContainerVariablesFields;
    use crate::schema_graphql::types::relation_type::RelationType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "PropertyContainerVariables")]
    pub struct RemoveProperty {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "PropertyContainerVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: $_type, propertyName: $property_name)]
        pub remove_property: RelationType,
    }

    pub fn remove_property_mutation<R: Into<RelationTypeId>>(ty: R, property_name: String) -> Operation<RemoveProperty, PropertyContainerVariables> {
        use cynic::MutationBuilder;
        RemoveProperty::build(PropertyContainerVariables::new(ty.into(), property_name))
    }
}
