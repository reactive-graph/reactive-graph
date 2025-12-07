#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyTypeDefinition;
    use crate::PropertyTypeDefinitions;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::PropertyType;
    use reactive_graph_graph::PropertyTypes;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct AddPropertiesVariables {
        pub id: UUID,
        pub properties: Option<Vec<PropertyTypeDefinition>>,
    }

    impl AddPropertiesVariables {
        pub fn new_property(id: Uuid, property: PropertyType) -> Self {
            Self {
                id: id.into(),
                properties: Some(vec![property.into()]),
            }
        }

        pub fn new_properties(id: Uuid, properties: PropertyTypes) -> Self {
            Self {
                id: id.into(),
                properties: Some(PropertyTypeDefinitions::from(properties).0),
            }
        }
    }
}
