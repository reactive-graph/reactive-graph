use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_CORE;

properties!(EventProperties, (EVENT, "event", ""));

component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
