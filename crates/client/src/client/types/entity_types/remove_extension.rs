#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod queries {
    use cynic::Operation;
    use cynic::QueryFragment;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    use crate::schema_graphql::types::entity_type::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct RemoveExtensionVariables {
        pub namespace: String,
        pub name: String,
        pub extension_namespace: String,
        pub extension_name: String,
    }

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "RemoveExtensionVariables")]
    pub struct RemoveExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemoveExtensionVariables")]
    pub struct MutationTypes {
        pub entities: MutationEntityTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "RemoveExtensionVariables")]
    pub struct MutationEntityTypes {
        #[arguments(type: { name: $name, namespace: $namespace }, extension: { name: $extension_name, namespace: $extension_namespace }
        )]
        pub remove_extension: EntityType,
    }

    pub fn remove_extension_mutation(
        ty: reactive_graph_graph::EntityTypeId,
        extension_ty: reactive_graph_graph::ExtensionTypeId,
    ) -> Operation<RemoveExtension, RemoveExtensionVariables> {
        use cynic::MutationBuilder;
        let vars = RemoveExtensionVariables {
            namespace: ty.namespace(),
            name: ty.type_name(),
            extension_namespace: extension_ty.namespace(),
            extension_name: extension_ty.type_name(),
        };
        RemoveExtension::build(vars)
    }

    pub fn remove_extension_with_variables(variables: RemoveExtensionVariables) -> Operation<RemoveExtension, RemoveExtensionVariables> {
        use cynic::MutationBuilder;
        RemoveExtension::build(variables)
    }
}
