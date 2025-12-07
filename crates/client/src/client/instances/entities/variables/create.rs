#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::PropertyInstanceDefinition;
    use crate::PropertyInstanceDefinitions;
    use crate::schema_graphql::scalar::UUID;
    use cynic::QueryVariables;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyInstances;
    use uuid::Uuid;

    #[derive(QueryVariables, Debug)]
    pub struct CreateEntityInstanceVariables {
        /// The fully qualified namespace of the entity type.
        #[cynic(rename = "type")]
        pub _type: String,
        /// The id of the entity instance. If none is given a random uuid will be generated.
        pub id: Option<UUID>,
        /// The description of the entity instance.
        pub description: Option<String>,
        pub properties: Option<Vec<PropertyInstanceDefinition>>,
    }

    impl CreateEntityInstanceVariables {
        pub fn new(ty: EntityTypeId, id: Option<Uuid>, description: Option<String>, properties: PropertyInstances) -> Self {
            let id = id.map(|id| id.into());
            let properties: PropertyInstanceDefinitions = properties.into();
            let properties = Some(properties.0);
            Self {
                _type: ty.namespace().to_string(),
                id,
                description,
                properties,
            }
        }
    }
}
