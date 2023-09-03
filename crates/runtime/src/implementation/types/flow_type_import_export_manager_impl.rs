use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;

use crate::api::FlowTypeImportExportManager;
use crate::api::FlowTypeManager;
use crate::di::component;
use crate::di::Component;
use crate::di::provides;
use crate::di::Wrc;
use crate::error::types::flow::FlowTypeExportError;
use crate::error::types::flow::FlowTypeImportError;
use crate::model::FlowType;
use crate::model::FlowTypeId;

#[component]
pub struct FlowTypeImportExportManagerImpl {
    flow_type_manager: Wrc<dyn FlowTypeManager>,
}

#[async_trait]
#[provides]
impl FlowTypeImportExportManager for FlowTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let flow_type: FlowType = serde_json::from_reader(reader)?;
        self.flow_type_manager.register(flow_type).map_err(FlowTypeImportError::RegistrationError)
    }

    fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError> {
        let Some(flow_type) = self.flow_type_manager.get(ty) else {
            return Err(FlowTypeExportError::FlowTypeNotFound(ty.clone()));
        };
        match File::create(path) {
            Ok(file) => serde_json::to_writer_pretty(&file, &flow_type).map_err(FlowTypeExportError::Serialization),
            Err(e) => Err(FlowTypeExportError::Io(e)),
        }
    }
}
