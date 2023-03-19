use crate::model::component_ty;
use crate::NAMESPACE_CORE;
use inexor_rgf_core_model::properties;

properties!(EventProperties, (EVENT, "event", ""));

component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
