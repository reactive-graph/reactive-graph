#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariables;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariablesFields;
    use reactive_graph_graph::ComponentExtensionTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "ExtensionContainerVariables")]
    pub struct RemoveExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "ExtensionContainerVariables")]
    pub struct MutationComponents {
        #[arguments(type: $_type, extension: $extension_type)]
        pub remove_extension: Component,
    }

    pub fn remove_extension_mutation<CE: Into<ComponentExtensionTypeId>>(
        component_extension_ty: CE,
    ) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(ExtensionContainerVariables::from(component_extension_ty.into()))
    }
}
