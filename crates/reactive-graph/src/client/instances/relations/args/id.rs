use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use crate::client::types::relations::args::parse_instance_id;
use crate::client::types::relations::args::parse_relation_ty;
use clap::Args;
use reactive_graph_graph::InstanceId;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationTypeId;
use std::fmt::Debug;
use std::fmt::Display;
use std::fmt::Formatter;
use uuid::Uuid;

/// CLI argument which identifies an relation instance by its id.
#[derive(Args, Debug, Clone)]
pub(crate) struct RelationInstanceIdArgs {
    /// The id of the outbound entity instance.
    #[clap(long)]
    pub outbound_id: Uuid,

    /// The fully qualified namespace of the relation type.
    #[clap(name = "relation_type", value_parser = parse_relation_ty)]
    pub relation_ty: RelationTypeId,

    /// The instance id.
    #[clap(value_parser = parse_instance_id)]
    pub instance_id: InstanceId,

    /// The id of the inbound entity instance.
    #[clap(short, long)]
    pub inbound_id: Uuid,
}

impl RelationInstanceIdArgs {
    pub fn not_found(&self) -> CommandError {
        let id: RelationInstanceId = self.into();
        NotFound(format!("The relation instance {} was not found", &id))
    }
}

impl From<&RelationInstanceIdArgs> for RelationInstanceId {
    fn from(id: &RelationInstanceIdArgs) -> Self {
        let ty = RelationInstanceTypeId::new(id.relation_ty.clone(), id.instance_id.clone());
        Self {
            outbound_id: id.outbound_id,
            ty,
            inbound_id: id.inbound_id,
        }
    }
}

impl Display for RelationInstanceIdArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let id = RelationInstanceId::from(self);
        std::fmt::Display::fmt(&id, f)
    }
}
