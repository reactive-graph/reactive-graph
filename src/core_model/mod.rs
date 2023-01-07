// TODO: Move this to a core model

use crate::model::component_ty;
use crate::model::entity_ty;

pub const NAMESPACE_CORE: &str = "core";
pub const NAMESPACE_LOGICAL: &str = "logical";
pub const NAMESPACE_FLOW: &str = "flow";

pub const PROPERTY_LABEL: &str = "label";
pub const PROPERTY_SHUTDOWN: &str = "shutdown";
pub const PROPERTY_TRIGGER: &str = "trigger";
pub const PROPERTY_RESULT: &str = "result";
pub const PROPERTY_EVENT: &str = "event";

component_ty!(COMPONENT_LABELED, NAMESPACE_CORE, COMPONENT_NAME_LABELED, "labeled");
component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
component_ty!(COMPONENT_ACTION, NAMESPACE_LOGICAL, COMPONENT_NAME_ACTION, "action");

entity_ty!(ENTITY_TYPE_SYSTEM_EVENT, NAMESPACE_CORE, ENTITY_TYPE_NAME_SYSTEM_EVENT, "system_event");
entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
entity_ty!(ENTITY_TYPE_GENERIC_FLOW, NAMESPACE_FLOW, ENTITY_TYPE_NAME_GENERIC_FLOW, "generic_flow");
