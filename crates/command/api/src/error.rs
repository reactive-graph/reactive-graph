use reactive_graph_graph::EntityTypeId;
use reactive_graph_reactive_service_api::ReactiveEntityRegistrationError;
use reactive_graph_type_system_api::EntityTypeRegistrationError;

#[derive(Debug)]
pub enum CommandRegistrationError {
    /// The reactive entity instance cannot be created.
    ReactiveEntityRegistrationError(ReactiveEntityRegistrationError),
    EntityTypeNotFound(EntityTypeId),
    EntityTypeRegistrationError(EntityTypeRegistrationError),
}
