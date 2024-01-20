use crate::NAMESPACE_CORE;
use inexor_rgf_graph::entity_ty;
use inexor_rgf_graph::properties;

properties!(ShutdownProperties, (DELAY, "delay", 0));

entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
