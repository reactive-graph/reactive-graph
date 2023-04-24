use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_CORE;

pub const PROPERTY_RESULT: &str = "result";

properties!(ActionProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_CORE, COMPONENT_NAME_ACTION, "action");

component_model!(Action, trigger);
