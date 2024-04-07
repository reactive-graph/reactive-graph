use crate::NAMESPACE_CORE;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;

properties!(LabeledProperties, (LABEL, "label", ""));

component_ty!(COMPONENT_LABELED, NAMESPACE_CORE, COMPONENT_NAME_LABELED, "labeled");
