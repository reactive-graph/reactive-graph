use crate::NAMESPACE_CORE;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;

properties!(LabeledProperties, (LABEL, "label", ""));

component_ty!(COMPONENT_LABELED, NAMESPACE_CORE, COMPONENT_NAME_LABELED, "labeled");
