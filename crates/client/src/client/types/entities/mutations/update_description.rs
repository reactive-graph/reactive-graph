#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariables;
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::EntityTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UpdateDescriptionVariables")]
    pub struct UpdateDescription {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: $_type, description: $description)]
        pub update_description: EntityType,
    }

    pub fn update_description_mutation<E: Into<EntityTypeId>>(ty: E, description: String) -> Operation<UpdateDescription, UpdateDescriptionVariables> {
        use cynic::MutationBuilder;
        UpdateDescription::build(UpdateDescriptionVariables::new(ty.into(), description))
    }
}
