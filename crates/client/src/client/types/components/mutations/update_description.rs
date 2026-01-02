#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariables;
    use crate::types::common::variables::update_description::variables::UpdateDescriptionVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "UpdateDescriptionVariables")]
    pub struct UpdateDescription {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "UpdateDescriptionVariables")]
    pub struct MutationComponents {
        #[arguments(type: $_type, description: $description)]
        pub update_description: Component,
    }

    pub fn update_description_mutation<C: Into<reactive_graph_graph::ComponentTypeId>>(
        ty: C,
        description: String,
    ) -> Operation<UpdateDescription, UpdateDescriptionVariables> {
        use cynic::MutationBuilder;
        UpdateDescription::build(UpdateDescriptionVariables::new(ty.into(), description))
    }
}
