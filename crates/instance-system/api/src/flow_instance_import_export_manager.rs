// use async_trait::async_trait;
// // use reactive_graph_core_plugins::EntityInstanceCreationError;
//
// // use crate::api::RelationInstanceCreationError;
// use reactive_graph_graph::FlowInstance;
//
// #[derive(Debug)]
// pub enum FlowInstanceCreationError {
//     EntityInstanceCreationError(EntityInstanceCreationError),
//     RelationInstanceCreationError(RelationInstanceCreationError),
// }
//
// impl From<EntityInstanceCreationError> for FlowInstanceCreationError {
//     fn from(e: EntityInstanceCreationError) -> Self {
//         FlowInstanceCreationError::EntityInstanceCreationError(e)
//     }
// }
//
// impl From<RelationInstanceCreationError> for FlowInstanceCreationError {
//     fn from(e: RelationInstanceCreationError) -> Self {
//         FlowInstanceCreationError::RelationInstanceCreationError(e)
//     }
// }
//
// #[derive(Debug)]
// pub enum FlowInstanceImportError {
//     Io(std::io::Error),
//     Deserialization(serde_json::Error),
//     FlowInstanceCreation(FlowInstanceCreationError),
// }
//
// impl From<std::io::Error> for FlowInstanceImportError {
//     fn from(e: std::io::Error) -> Self {
//         FlowInstanceImportError::Io(e)
//     }
// }
//
// impl From<serde_json::Error> for FlowInstanceImportError {
//     fn from(e: serde_json::Error) -> Self {
//         FlowInstanceImportError::Deserialization(e)
//     }
// }
//
// impl From<FlowInstanceCreationError> for FlowInstanceImportError {
//     fn from(e: FlowInstanceCreationError) -> Self {
//         FlowInstanceImportError::FlowInstanceCreation(e)
//     }
// }
//
// #[async_trait]
// pub trait FlowInstanceManager: Send + Sync {
//     fn create(&self, flow_instance: FlowInstance) -> Result<FlowInstance, FlowInstanceCreationError>;
//
//     fn commit(&self, flow_instance: FlowInstance);
//
//     fn delete(&self, flow_instance: FlowInstance);
//
//     fn import(&self, path: &str) -> Result<FlowInstance, FlowInstanceImportError>;
//
//     fn export(&self, flow_instance: FlowInstance, path: &str);
// }
