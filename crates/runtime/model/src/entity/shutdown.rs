use crate::NAMESPACE_CORE;
use reactive_graph_graph::entity_ty;
use reactive_graph_graph::properties;

properties!(ShutdownProperties, (DELAY, "delay", 0));

entity_ty!(ENTITY_TYPE_SHUTDOWN, NAMESPACE_CORE, ENTITY_TYPE_NAME_SHUTDOWN, "shutdown");
