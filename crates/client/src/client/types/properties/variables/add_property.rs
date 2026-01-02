#[cynic::schema_for_derives(file = r#"../../schema/graphql/reactive-graph-schema.graphql"#, module = "crate::schema_graphql::schema")]
pub mod variables {
    use crate::ExtensionDefinitions;
    use crate::PropertyTypeDefinition;
    use cynic::QueryVariables;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::PropertyType;

    #[derive(QueryVariables, Debug)]
    pub struct AddPropertyVariables {
        /// The fully qualified namespace of the type.
        #[cynic(rename = "type")]
        pub _type: String,
        pub property: PropertyTypeDefinition,
    }

    impl AddPropertyVariables {
        pub fn new<TY: NamespacedTypeGetter>(ty: TY, property_type: PropertyType) -> Self {
            let extensions: ExtensionDefinitions = property_type.extensions.into();
            Self {
                _type: ty.namespace().to_string(),
                property: PropertyTypeDefinition {
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
