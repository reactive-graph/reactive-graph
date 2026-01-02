use async_graphql::dynamic::Interface;
use async_trait::async_trait;
use reactive_graph_graph::Component;
use reactive_graph_lifecycle::Lifecycle;
use springtime_di::injectable;

pub const INTERFACE_ENTITY: &str = "Entity";
pub const INTERFACE_ENTITY_FIELD_ID: &str = "id";

pub const INTERFACE_RELATION: &str = "Relation";
pub const INTERFACE_RELATION_FIELD_ID: &str = "id";
pub const INTERFACE_RELATION_FIELD_INSTANCE_ID: &str = "instance_id";

pub const INTERFACE_FLOW: &str = "Flow";
pub const INTERFACE_FLOW_FIELD_ID: &str = "id";

#[injectable]
#[async_trait]
pub trait InterfaceManager: Send + Sync + Lifecycle {
    /// Returns the interfaces of the dynamic graph.
    fn get_interfaces(&self) -> Vec<Interface>;

    /// Constructs an interface for the given component.
    fn get_component_interface(&self, component: Component) -> Interface;

    /// Constructs an interface for entity types. Each entity type has an id field.
    fn get_entity_interface(&self) -> Interface;

    /// Constructs an interface for relation types. Each relation type has fields for the relation
    /// type and the relation type instance id.
    fn get_relation_interface(&self) -> Interface;

    /// Constructs an interface for flow types. Each flow type has an id field which returns the id
    /// of the wrapper entity instance.
    fn get_flow_interface(&self) -> Interface;
}
