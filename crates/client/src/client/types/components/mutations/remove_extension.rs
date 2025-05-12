#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use cynic::Operation;
    use cynic::QueryFragment;

    use crate::schema_graphql::types::component::Component;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariables;
    use crate::types::extensions::variables::container::variables::ExtensionContainerVariablesFields;
    use reactive_graph_graph::NamespacedTypeGetter;

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
        #[arguments(type: { name: $name, namespace: $namespace }, extension: { name: $extension_name, namespace: $extension_namespace }
        )]
        pub remove_extension: Component,
    }

    pub fn remove_extension_mutation(
        ty: reactive_graph_graph::ComponentTypeId,
        extension_ty: reactive_graph_graph::ExtensionTypeId,
    ) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        let vars = ExtensionContainerVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            extension_namespace: extension_ty.namespace(),
            extension_name: extension_ty.type_name(),
        };
        RemoveExtension::build(vars)
    }

    pub fn remove_extension_with_variables(variables: ExtensionContainerVariables) -> Operation<RemoveExtension, ExtensionContainerVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(variables)
    }
}
