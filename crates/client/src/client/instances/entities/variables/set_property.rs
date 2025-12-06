#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {

    use crate::PropertyInstanceDefinition;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use serde_json::Value;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct SetPropertyVariables {
        pub id: UUID,
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    impl SetPropertyVariables {
        pub fn new(id: Uuid, name: String, value: Value) -> Self {
            Self {
                id: id.into(),
                properties: Some(vec![PropertyInstanceDefinition { name, value }]),
            }
        }
    }
}
