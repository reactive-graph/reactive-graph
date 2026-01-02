#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod mutations {
    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariables;
    use crate::client::types::extensions::variables::add_extension::variables::AddExtensionVariablesFields;
    use crate::schema_graphql::types::relation_type::RelationType;
    use cynic::Operation;
    use cynic::QueryFragment;
    use reactive_graph_graph::Extension;
    use reactive_graph_graph::RelationTypeId;

    #[derive(cynic::QueryFragment, Debug)]
    #[cynic(graphql_type = "Mutation", variables = "AddExtensionVariables")]
    pub struct AddExtension {
        pub types: MutationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationTypes {
        pub relations: MutationRelationTypes,
    }

    #[derive(QueryFragment, Debug)]
    #[cynic(variables = "AddExtensionVariables")]
    pub struct MutationRelationTypes {
        #[arguments(type: $_type, extension: $extension)]
        pub add_extension: RelationType,
    }

    pub fn add_extension_mutation<R: Into<RelationTypeId>, EXT: Into<Extension>>(ty: R, extension: EXT) -> Operation<AddExtension, AddExtensionVariables> {
        use cynic::MutationBuilder;
        AddExtension::build(AddExtensionVariables::new(ty.into(), extension.into()))
    }
}
