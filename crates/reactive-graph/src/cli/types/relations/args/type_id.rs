use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::relations::type_id::queries::RelationTypeIdVariables;
use reactive_graph_graph::RelationTypeId;

/// The relation type.
#[derive(Args, Debug, Clone)]
pub(crate) struct RelationTypeIdArgs {
    /// The relation type namespace.
    pub namespace: String,

    /// The relation type name.
    pub name: String,
}

impl RelationTypeIdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("RelationType {}__{} not found", &self.namespace, &self.name))
    }
}

impl From<RelationTypeIdArgs> for RelationTypeId {
    fn from(ty: RelationTypeIdArgs) -> Self {
        RelationTypeId::new_from_type(ty.namespace, ty.name)
    }
}

impl From<&RelationTypeIdArgs> for RelationTypeIdVariables {
    fn from(ty: &RelationTypeIdArgs) -> Self {
        let ty: RelationTypeId = ty.clone().into();
        ty.into()
    }
}
