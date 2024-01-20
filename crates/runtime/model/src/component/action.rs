use crate::NAMESPACE_CORE;
use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

pub const PROPERTY_RESULT: &str = "result";

properties!(ActionProperties, (TRIGGER, "trigger", false));

component_ty!(COMPONENT_ACTION, NAMESPACE_CORE, COMPONENT_NAME_ACTION, "action");

component_model!(Action, trigger);
