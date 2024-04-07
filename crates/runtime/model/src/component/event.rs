use crate::NAMESPACE_CORE;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(EventProperties, (EVENT, "event", ""));

component_ty!(COMPONENT_EVENT, NAMESPACE_CORE, COMPONENT_NAME_EVENT, "event");
