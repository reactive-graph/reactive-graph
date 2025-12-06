#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::relation_type::RelationType;
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariables;
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UpdateDescriptionVariables")]
    pub struct UpdateDescription {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: $_type, description: $description)]
        pub update_description: RelationType,
    }

    pub fn update_description_mutation<R: Into<RelationTypeId>>(ty: R, description: String) -> Operation<UpdateDescription, UpdateDescriptionVariables> {
        use cynic::MutationBuilder;
        UpdateDescription::build(UpdateDescriptionVariables::new(ty.into(), description))
    }
}
