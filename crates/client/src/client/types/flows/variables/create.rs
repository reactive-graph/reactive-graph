#[cynic::schema_for_derives(file = r#"schema_graphql.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinition;
    use crate::PropertyTypeDefinition;
    use crate::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
    use cynic::QueryVariables;
    use typed_builder::TypedBuilder;

    #[derive(QueryVariables, Debug, TypedBuilder)]
    pub struct CreateFlowTypeVariables {
        pub namespace: String,
        pub name: String,
        #[builder(default)]
        pub description: Option<String>,
        pub wrapper_entity_instance: EntityInstanceDefinition,
        #[builder(default)]
        pub variables: Option<Vec<PropertyTypeDefinition>>,
        #[builder(default)]
        pub extensions: Option<Vec<ExtensionDefinition>>,
    }
}
