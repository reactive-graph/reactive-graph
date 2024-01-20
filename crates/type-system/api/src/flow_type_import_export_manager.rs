use async_trait::async_trait;
use springtime_di::injectable;

use inexor_rgf_graph::FlowType;
use inexor_rgf_graph::FlowTypeId;
use inexor_rgf_lifecycle::Lifecycle;

use crate::FlowTypeExportError;
use crate::FlowTypeImportError;

#[injectable]
#[async_trait]
pub trait FlowTypeImportExportManager: Send + Sync + Lifecycle {
    /// Imports a flow type from a JSON file file located at the given path.
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given name to a JSON file located at the given path.
    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError>;
}
