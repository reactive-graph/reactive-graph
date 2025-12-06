#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariables;
    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariablesFields;
    use crate::schema_graphql::types::component::Component;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::Extension;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddExtensionVariables")]
    pub struct AddExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationTypes {
        pub components: MutationComponents,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationComponents {
        #[arguments(type: $_type, extension: $extension)]
        pub add_extension: Component,
    }

    pub fn add_extension_mutation<C: Into<reactive_graph_graph::ComponentTypeId>, EXT: Into<Extension>>(
        ty: C,
        extension: EXT,
    ) -> Operation<AddExtension, AddExtensionVariables> {
        use cynic::MutationBuilder;
        AddExtension::build(AddExtensionVariables::new(ty.into(), extension.into()))
    }
}
