use crate::client::instances::flows::args::add_entity_instance::AddEntityInstanceArgs;
use crate::client::types::flows::args::type_id::FlowTypeIdArgs;
use clap::Args;
use reactive_graph_client::PropertyInstanceDefinitions;
use reactive_graph_client::schema_graphql::instances::entity_instance::EntityInstanceDefinition;
use reactive_graph_client::types::flows::variables::add_entity_instance::variables::AddEntityInstanceVariables;
use uuid::Uuid;

#[derive(Args, Debug, Clone)]
pub(crate) struct FlowTypeAddEntityInstanceArgs {
    /// The flow type.
    #[clap(flatten)]
    pub ty: FlowTypeIdArgs,

    /// The entity instance to add.
    #[clap(flatten)]
    pub entity_instance: AddEntityInstanceArgs,
}

impl From<&FlowTypeAddEntityInstanceArgs> for AddEntityInstanceVariables {
    fn from(args: &FlowTypeAddEntityInstanceArgs) -> Self {
        AddEntityInstanceVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            entity_instance: EntityInstanceDefinition {
                namespace: args.entity_instance.entity_type_namespace.clone(),
                type_name: args.entity_instance.entity_type_name.clone(),
                id: args.entity_instance.id.unwrap_or_else(Uuid::new_v4).into(),
                description: args.entity_instance.description.clone().unwrap_or_default(),
                properties: PropertyInstanceDefinitions::from(args.entity_instance.properties()).into(),
                extensions: vec![],
            },
        }
    }
}
