use crate::model::entity_ty;
use crate::model::properties;
use crate::NAMESPACE_CORE;

properties!(ShutdownProperties, (DELAY, "delay", 0));

entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
