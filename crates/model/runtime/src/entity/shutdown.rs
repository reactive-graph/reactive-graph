use crate::model::entity_ty;
use crate::NAMESPACE_CORE;
use inexor_rgf_core_model::properties;

properties!(ShutdownProperties, (SHUTDOWN, "shutdown", ""));

entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
