use async_trait::async_trait;

use crate::model::Flow;

#[derive(Debug)]
pub struct FlowCreationError;

#[derive(Debug)]
pub struct FlowImportError;

#[async_trait]
pub trait FlowManager: Send + Sync {
    fn create(&self, flow: Flow) -> Result<Flow, FlowCreationError>;

    fn commit(&self, flow: Flow);

    fn delete(&self, flow: Flow);

    fn import(&self, path: String) -> Result<Flow, FlowImportError>;

    fn export(&self, flow: Flow, path: String);
}
