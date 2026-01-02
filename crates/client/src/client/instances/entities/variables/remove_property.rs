#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct RemovePropertiesVariables {
        pub id: UUID,
        pub properties: Option<Vec<String>>,
    }

    impl RemovePropertiesVariables {
        pub fn new_by_name<S: Into<String>>(id: Uuid, property_name: S) -> Self {
            Self {
                id: id.into(),
                properties: Some(vec![property_name.into()]),
            }
        }

        pub fn new_by_type<P: Into<PropertyType>>(id: Uuid, property_type: P) -> Self {
            Self {
                id: id.into(),
                properties: Some(vec![property_type.into().name]),
            }
        }

        pub fn new_by_names(id: Uuid, property_names: Vec<String>) -> Self {
            Self {
                id: id.into(),
                properties: Some(property_names),
            }
        }

        pub fn new_by_types(id: Uuid, property_types: &PropertyTypes) -> Self {
            Self {
                id: id.into(),
                properties: Some(property_types.iter().map(|property_type| property_type.name.clone()).collect()),
            }
        }
    }
}
