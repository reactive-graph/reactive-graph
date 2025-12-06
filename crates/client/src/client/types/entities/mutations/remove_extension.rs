#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::schema_graphql::types::entity_type::EntityType;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariables;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariablesFields;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::entity_extension_type_id::EntityExtensionTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExtensionContainerVariables")]
    pub struct RemoveExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: $_type, extension: $extension_type)]
        pub remove_extension: EntityType,
    }

    pub fn remove_extension_mutation<EE: Into<EntityExtensionTypeId>>(entity_extension_ty: EE) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(ExtensionContainerVariables::from(entity_extension_ty.into()))
    }
}
