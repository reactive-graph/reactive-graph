#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::relation_type::RelationType;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariables;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::RelationExtensionTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExtensionContainerVariables")]
    pub struct RemoveExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: $_type, extension: $extension_type)]
        pub remove_extension: RelationType,
    }

    pub fn remove_extension_mutation<RE: Into<RelationExtensionTypeId>>(relation_extension_ty: RE) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(ExtensionContainerVariables::from(relation_extension_ty.into()))
    }
}
