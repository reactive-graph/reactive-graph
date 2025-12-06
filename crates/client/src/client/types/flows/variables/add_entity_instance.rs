#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
    use cynic::QueryVariables;
    use reactive_graph_graph::EntityInstance;
    use reactive_graph_graph::NamespacedTypeGetter;

    #[derive(QueryVariables, Debug)]
    pub struct AddEntityInstanceVariables {
        #[cynic(rename = "type")]
        pub _type: String,
        pub entity_instance: EntityInstanceDefinition,
    }

    impl AddEntityInstanceVariables {
        pub fn new(ty: reactive_graph_graph::FlowTypeId, entity_instance: EntityInstance) -> Self {
            Self {
                _type: ty.namespace().to_string(),
                entity_instance: EntityInstanceDefinition::from(entity_instance),
            }
        }
    }
}
