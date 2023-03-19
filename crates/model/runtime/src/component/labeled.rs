use crate::model::component_ty;
use crate::model::properties;
use crate::NAMESPACE_CORE;

properties!(LabeledProperties, (LABEL, "label", ""));

component_ty!(COMPONENT_LABELED, NAMESPACE_CORE, COMPONENT_NAME_LABELED, "labeled");
