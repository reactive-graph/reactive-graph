use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::entity_types::type_id::queries::EntityTypeIdVariables;
use reactive_graph_graph::EntityTypeId;

/// The entity type.
#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeIdArgs {
    /// The entity type namespace.
    pub namespace: String,

    /// The entity type name.
    pub name: String,
}

impl EntityTypeIdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("EntityType {}__{} not found", &self.namespace, &self.name))
    }
}

impl From<EntityTypeIdArgs> for EntityTypeId {
    fn from(ty: EntityTypeIdArgs) -> Self {
        EntityTypeId::new_from_type(ty.namespace, ty.name)
    }
}

impl From<&EntityTypeIdArgs> for EntityTypeIdVariables {
    fn from(ty: &EntityTypeIdArgs) -> Self {
        let ty: EntityTypeId = ty.clone().into();
        ty.into()
    }
}
