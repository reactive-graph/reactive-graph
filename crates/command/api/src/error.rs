use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_reactive_service_api::ReactiveEntityRegistrationError;
use inexor_rgf_type_system_api::EntityTypeRegistrationError;

#[derive(Debug)]
pub enum CommandRegistrationError {
    /// The reactive entity instance cannot be created.
    ReactiveEntityRegistrationError(ReactiveEntityRegistrationError),
    EntityTypeNotFound(EntityTypeId),
    EntityTypeRegistrationError(EntityTypeRegistrationError),
}
