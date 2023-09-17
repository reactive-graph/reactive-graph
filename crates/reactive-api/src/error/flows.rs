use thiserror::Error;
use uuid::Uuid;

#[derive(Debug, Error)]
pub enum ReactiveFlowConstructionError {
    #[error("Missing the wrapper entity instance. Check if an entity instance exists with the same id as the flow id")]
    MissingWrapperInstance,
    #[error("The outbound entity instance {0} cannot be found")]
    MissingOutboundEntityInstance(Uuid),
    #[error("The inbound entity instance {0} cannot be found")]
    MissingInboundEntityInstance(Uuid),
}
