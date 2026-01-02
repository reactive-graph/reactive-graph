#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinitions;
    use crate::PropertyTypeDefinition;
    use cynic::QueryVariables;
    use reactive_graph_graph::FlowTypeId;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyType;

    #[derive(QueryVariables, Debug)]
    pub struct AddVariableVariables {
        /// The fully qualified namespace of the flow type.
        #[cynic(rename = "type")]
        pub _type: String,
        pub variable: PropertyTypeDefinition,
    }

    impl AddVariableVariables {
        pub fn new<FT: Into<FlowTypeId>, PT: Into<PropertyType>>(ty: FT, property_type: PT) -> Self {
            let ty = ty.into();
            let property_type = property_type.into();
            let extensions: ExtensionDefinitions = property_type.extensions.into();
            Self {
                _type: ty.namespace().to_string(),
                variable: PropertyTypeDefinition {
                    name: property_type.name,
                    data_type: property_type.data_type.into(),
                    description: property_type.description,
                    socket_type: property_type.socket_type.into(),
                    mutability: property_type.mutability.into(),
                    extensions: extensions.0,
                },
            }
        }
    }
}
