use async_graphql::InputObject;
use uuid::Uuid;

use reactive_graph_graph::InstanceId;
use reactive_graph_graph::InstanceIdError;
use reactive_graph_graph::NamespaceSegment;
use reactive_graph_graph::RelationInstanceTypeId;
use reactive_graph_graph::RelationInstanceTypeIdError;
use reactive_graph_graph::RelationTypeId;

/// The
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "PropertyConnectorOutboundInbound")]
pub struct GraphQLPropertyConnectorOutboundInbound {
    /// The id of the outbound/inbound entity instance.
    pub id: Uuid,

    /// The property name of the outbound/inbound entity instance.
    pub property_name: String,
}

/// Constructs the instance_id.
///
/// Between two nodes only one edge with the same type can exist. Therefore, we construct a unique
/// type which contains the names of the outbound property and the inbound property. This allows
/// *exactly one* connector (of the given connector type) between the two properties.
#[derive(Debug, Clone, InputObject)]
#[graphql(name = "PropertyConnectorId")]
pub struct GraphQLPropertyConnectorId {
    /// The outbound entity instance and property.
    pub outbound: GraphQLPropertyConnectorOutboundInbound,

    /// The fully qualified namespace of the relation type.
    /// TODO: REGEX
    #[graphql(name = "name")]
    pub namespace: String,

    /// The inbound entity instance and property.
    pub inbound: GraphQLPropertyConnectorOutboundInbound,
}

impl GraphQLPropertyConnectorId {
    pub fn instance_id(&self) -> Result<InstanceId, RelationInstanceTypeIdError> {
        Ok(InstanceId::new_segmented(vec![
            NamespaceSegment::try_from(&self.outbound.property_name)
                .map_err(InstanceIdError::NamespaceSegmentError)
                .map_err(RelationInstanceTypeIdError::InstanceIdError)?,
            NamespaceSegment::try_from(&self.inbound.property_name)
                .map_err(InstanceIdError::NamespaceSegmentError)
                .map_err(RelationInstanceTypeIdError::InstanceIdError)?,
        ]))
    }

    pub fn parse(&self) -> Result<RelationInstanceTypeId, RelationInstanceTypeIdError> {
        Ok(RelationInstanceTypeId::new(
            RelationTypeId::parse_namespace(&self.namespace).map_err(RelationInstanceTypeIdError::NamespacedTypeError)?,
            self.instance_id()?,
        ))
    }
}
