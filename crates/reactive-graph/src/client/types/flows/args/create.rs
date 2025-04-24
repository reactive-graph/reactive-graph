use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
use reactive_graph_client::schema_graphql::scalar::UUID;
use reactive_graph_client::types::flows::variables::create::variables::CreateFlowTypeVariables;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateFlowTypeArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The namespace of the entity type of the wrapper entity instance.
    pub entity_type_namespace: String,

    /// The type name of the entity type of the wrapper entity instance.
    pub entity_type_name: String,

    /// The id of the wrapper entity instance.
    pub wrapper_entity_instance_id: Uuid,

    /// The flow type description.
    pub description: Option<String>,

    /// The description of the wrapper entity instance.
    pub wrapper_entity_instance_description: Option<String>,
    // /// The entity instance properties.
    // #[clap(short, long, value_parser = parse_property)]
    // pub properties: Option<Vec<(String, Value)>>,
    //
    // /// The variables of the flow type.
    // #[clap(short, long, value_parser = parse_property)]
    // pub variables: Option<Vec<(String, Value)>>,
}

impl From<&CreateFlowTypeArgs> for CreateFlowTypeVariables {
    fn from(args: &CreateFlowTypeArgs) -> Self {
        // let variables = match &args.variables {
        //     None => Vec::new(),
        //     Some(variables) => variables
        //         .iter()
        //         .map(PropertyInstanceDefinition::from)
        //         // .map(|(name, value)| PropertyInstanceDefinition {
        //         //     name: name.clone(),
        //         //     value: value.clone(),
        //         // })
        //         .collect(),
        // };
        let wrapper_entity_instance = EntityInstanceDefinition {
            namespace: args.entity_type_namespace.clone(),
            type_name: args.entity_type_name.clone(),
            id: UUID(args.wrapper_entity_instance_id),
            description: args.wrapper_entity_instance_description.clone().unwrap_or_default(),
            properties: Vec::new(),
            extensions: Vec::new(),
        };
        CreateFlowTypeVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
            wrapper_entity_instance,
            variables: None,
            extensions: None,
        }
    }
}
