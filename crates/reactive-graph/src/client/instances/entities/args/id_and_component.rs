use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::types::components::args::parse_component_ty;
use clap::Args;
use reactive_graph_graph::ComponentTypeId;
use uuid::Uuid;

/// Identifies a component of an entity instance.
#[derive(Args, Debug, Clone)]
pub(crate) struct IdAndComponentArgs {
    /// The id of the reactive instance.
    pub id: Uuid,

    /// The fully qualified namespace of the component.
    #[clap(name = "component", value_parser = parse_component_ty)]
    pub component_ty: ComponentTypeId,
}

impl IdAndComponentArgs {
    pub fn id_not_found(&self) -> CommandError {
        NotFound(format!("The instance with the id {} was not found", &self.id))
    }
}
