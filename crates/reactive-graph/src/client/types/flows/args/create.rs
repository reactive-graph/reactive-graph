use crate::client::types::flows::args::parse_flow_ty;
use clap::Args;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateFlowTypeArgs {
    /// The fully qualified namespace of the flow type.
    #[clap(name = "flow_type", value_parser = parse_flow_ty)]
    pub flow_ty: FlowTypeId,

    /// The fully qualified namespace of the entity type of the wrapper entity instance.
    pub entity_ty: EntityTypeId,

    /// The id of the wrapper entity instance.
    pub wrapper_entity_instance_id: Uuid,

    /// The flow type description.
    #[clap(short, long)]
    pub description: Option<String>,

    /// The description of the wrapper entity instance.
    #[clap(short = 'D', long = "wrapper_description")]
    pub wrapper_entity_instance_description: Option<String>,
    // /// The entity instance properties.
    // #[clap(short, long, value_parser = parse_property)]
    // pub properties: Option<Vec<(String, Value)>>,
    //
    // /// The variables of the flow type.
    // #[clap(short, long, value_parser = parse_property)]
    // pub variables: Option<Vec<(String, Value)>>,
}

// impl From<&CreateFlowTypeArgs> for CreateFlowTypeVariables {
//     fn from(args: &CreateFlowTypeArgs) -> Self {
//         // let variables = match &args.variables {
//         //     None => Vec::new(),
//         //     Some(variables) => variables
//         //         .iter()
//         //         .map(PropertyInstanceDefinition::from)
//         //         // .map(|(name, value)| PropertyInstanceDefinition {
//         //         //     name: name.clone(),
//         //         //     value: value.clone(),
//         //         // })
//         //         .collect(),
//         // };
//         let wrapper_entity_instance = EntityInstanceDefinition {
//             namespace: args.entity_type_namespace.clone(),
//             type_name: args.entity_type_name.clone(),
//             id: UUID(args.wrapper_entity_instance_id),
//             description: args.wrapper_entity_instance_description.clone().unwrap_or_default(),
//             properties: Vec::new(),
//             extensions: Vec::new(),
//         };
//         CreateFlowTypeVariables {
//             namespace: args.ty.namespace.clone(),
//             name: args.ty.name.clone(),
//             description: args.description.clone(),
//             wrapper_entity_instance,
//             variables: None,
//             extensions: None,
//         }
//     }
// }
