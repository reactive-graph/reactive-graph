use crate::model::component_ty;

pub const NAMESPACE_TRIGGER: &str = "trigger";
pub const PROPERTY_TRIGGER: &str = "trigger";
pub const PROPERTY_RESULT: &str = "result";

component_ty!(COMPONENT_ACTION, NAMESPACE_TRIGGER, COMPONENT_NAME_ACTION, "action");
