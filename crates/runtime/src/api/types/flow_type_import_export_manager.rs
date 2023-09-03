use async_trait::async_trait;

use crate::error::types::flow::FlowTypeExportError;
use crate::error::types::flow::FlowTypeImportError;
use crate::model::FlowType;
use crate::model::FlowTypeId;

#[async_trait]
pub trait FlowTypeImportExportManager: Send + Sync {
    /// Imports a flow type from a JSON file file located at the given path.
    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given name to a JSON file located at the given path.
    fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError>;
}
