use crate::NAMESPACE_CORE;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(EventProperties, (EVENT, "event", ""));

component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
