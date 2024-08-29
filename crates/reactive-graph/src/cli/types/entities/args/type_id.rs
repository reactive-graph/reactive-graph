use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::entities::type_id::queries::EntityTypeIdVariables;
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

/// The outbound entity type.
#[derive(Args, Debug, Clone)]
pub(crate) struct OutboundEntityTypeIdArgs {
    /// The outbound entity type namespace.
    pub outbound_type_namespace: String,

    /// The outbound entity type name.
    pub outbound_type_name: String,
}

/// The inbound entity type.
#[derive(Args, Debug, Clone)]
pub(crate) struct InboundEntityTypeIdArgs {
    /// The inbound entity type namespace.
    pub inbound_type_namespace: String,

    /// The inbound entity type name.
    pub inbound_type_name: String,
}

/// The entity type as option.
#[derive(Args, Debug, Clone)]
pub(crate) struct EntityTypeIdOptions {
    /// The entity type namespace.
    #[clap(long)]
    pub namespace: Option<String>,

    /// The entity type name.
    #[clap(short, long)]
    pub name: Option<String>,
}

impl From<EntityTypeIdOptions> for Option<EntityTypeId> {
    fn from(ty: EntityTypeIdOptions) -> Self {
        if ty.namespace.is_none() && ty.name.is_none() {
            return None;
        }
        Some(EntityTypeId::new_from_type(ty.namespace.unwrap_or_default(), ty.name.unwrap_or_default()))
    }
}
