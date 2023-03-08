pub use instance_info::*;

pub mod instance_info;

// TODO: Move this to a core model

use crate::model::component_ty;
use crate::model::entity_ty;
use crate::model::extension_ty;

pub const NAMESPACE_CORE: &str = "core";
pub const NAMESPACE_TRIGGER: &str = "trigger";
pub const NAMESPACE_FLOW: &str = "flow";
pub const NAMESPACE_DYNAMIC_GRAPH: &str = "dynamic_graph";

pub const PROPERTY_LABEL: &str = "label";
pub const PROPERTY_SHUTDOWN: &str = "shutdown";
pub const PROPERTY_TRIGGER: &str = "trigger";
pub const PROPERTY_RESULT: &str = "result";
pub const PROPERTY_EVENT: &str = "event";

component_ty!(COMPONENT_LABELED, NAMESPACE_CORE, COMPONENT_NAME_LABELED, "labeled");
component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
component_ty!(COMPONENT_ACTION, NAMESPACE_TRIGGER, COMPONENT_NAME_ACTION, "action");

entity_ty!(ENTITY_TYPE_SYSTEM_EVENT, NAMESPACE_CORE, ENTITY_TYPE_NAME_SYSTEM_EVENT, "system_event");
entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
entity_ty!(ENTITY_TYPE_GENERIC_FLOW, NAMESPACE_FLOW, ENTITY_TYPE_NAME_GENERIC_FLOW, "generic_flow");

extension_ty!(EXTENSION_DIVERGENT, NAMESPACE_CORE, EXTENSION_NAME_DIVERGENT, "divergent");
extension_ty!(EXTENSION_COMPONENT_CATEGORY, NAMESPACE_CORE, EXTENSION_NAME_COMPONENT_CATEGORY, "component_category");
extension_ty!(EXTENSION_ENTITY_TYPE_CATEGORY, NAMESPACE_CORE, EXTENSION_NAME_ENTITY_TYPE_CATEGORY, "entity_type_category");
extension_ty!(
    EXTENSION_RELATION_TYPE_CATEGORY,
    NAMESPACE_CORE,
    EXTENSION_NAME_RELATION_TYPE_CATEGORY,
    "relation_type_category"
);
extension_ty!(EXTENSION_FLOW_TYPE_CATEGORY, NAMESPACE_CORE, EXTENSION_NAME_FLOW_TYPE_CATEGORY, "flow_type_category");

extension_ty!(
    EXTENSION_FLOW_RESOLVE_EXISTING_INSTANCE,
    NAMESPACE_FLOW,
    EXTENSION_NAME_FLOW_RESOLVE_EXISTING_INSTANCE,
    "resolve_existing_instance"
);

extension_ty!(
    EXTENSION_FLOW_UUID_TYPE_VARIABLE,
    NAMESPACE_FLOW,
    EXTENSION_NAME_FLOW_UUID_TYPE_VARIABLE,
    "uuid_type_variable"
);

extension_ty!(
    EXTENSION_FLOW_UUID_TYPE_EXTENSION,
    NAMESPACE_FLOW,
    EXTENSION_NAME_FLOW_UUID_TYPE_EXTENSION,
    "uuid_type_extension"
);

extension_ty!(EXTENSION_FIELD_NAME, NAMESPACE_DYNAMIC_GRAPH, EXTENSION_NAME_FIELD_NAME, "field_name");
extension_ty!(EXTENSION_FIELD_DESCRIPTION, NAMESPACE_DYNAMIC_GRAPH, EXTENSION_NAME_FIELD_DESCRIPTION, "field_description");
