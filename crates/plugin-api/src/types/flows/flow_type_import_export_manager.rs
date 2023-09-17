use crate::model::FlowType;
use crate::model::FlowTypeId;
use async_trait::async_trait;
use inexor_rgf_rt_api::FlowTypeExportError;
use inexor_rgf_rt_api::FlowTypeImportError;

#[async_trait]
pub trait FlowTypeImportExportManager: Send + Sync {
    /// Imports a flow type from a JSON file located at the given path.
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given type id to a JSON file located at the given path.
    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError>;
}
