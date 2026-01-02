#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::flow_type::FlowType;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariables;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariablesFields;
    use reactive_graph_graph::FlowExtensionTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExtensionContainerVariables")]
    pub struct RemoveExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationTypes {
        pub flows: MutationFlowTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationFlowTypes {
        #[arguments(type: $_type, extension: $extension_type)]
        pub remove_extension: FlowType,
    }

    pub fn remove_extension_mutation<FE: Into<FlowExtensionTypeId>>(flow_extension_ty: FE) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(ExtensionContainerVariables::from(flow_extension_ty.into()))
    }
}
