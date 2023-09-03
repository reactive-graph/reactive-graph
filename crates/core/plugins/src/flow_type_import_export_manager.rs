use crate::model::FlowType;
use crate::model::FlowTypeId;

#[derive(Debug)]
pub struct FlowTypeImportError;

#[derive(Debug)]
pub struct FlowTypeExportError;

pub trait FlowTypeImportExportManager: Send + Sync {
    /// Imports a flow type from a JSON file located at the given path.
    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError>;

    /// Exports the flow type with the given type id to a JSON file located at the given path.
    fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError>;
}
