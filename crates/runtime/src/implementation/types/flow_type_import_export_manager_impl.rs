use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use inexor_rgf_rt_api::error::types::serde::DeserializationError;
use inexor_rgf_rt_api::error::types::serde::SerializationError;

use crate::api::FlowTypeImportExportManager;
use crate::api::FlowTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::FlowType;
use crate::model::FlowTypeId;
use crate::rt_api::FlowTypeExportError;
use crate::rt_api::FlowTypeImportError;

#[component]
pub struct FlowTypeImportExportManagerImpl {
    flow_type_manager: Wrc<dyn FlowTypeManager>,
}

#[async_trait]
#[provides]
impl FlowTypeImportExportManager for FlowTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let flow_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<FlowType>(&content).map_err(|e| DeserializationError::Json(e).into()),
            Some("json5") => json5::from_str::<FlowType>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            Some("toml") => toml::from_str::<FlowType>(&content).map_err(|e| DeserializationError::Toml(e).into()),
            Some(ext) => Err(FlowTypeImportError::UnsupportedFormat(ext.to_string())),
            None => Err(FlowTypeImportError::UnsupportedFormat(Default::default())),
        }?;
        self.flow_type_manager.register(flow_type).map_err(FlowTypeImportError::RegistrationError)
    }

    async fn export(&self, ty: &FlowTypeId, path: &str) -> Result<(), FlowTypeExportError> {
        let Some(flow_type) = self.flow_type_manager.get(ty) else {
            return Err(FlowTypeExportError::FlowTypeNotFound(ty.clone()));
        };
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => match File::create(path) {
                Ok(file) => serde_json::to_writer_pretty(&file, &flow_type).map_err(|e| SerializationError::Json(e).into()),
                Err(e) => Err(FlowTypeExportError::Io(e)),
            },
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&flow_type).map_err(|e| FlowTypeExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(FlowTypeExportError::Io)
                }
                Err(e) => Err(FlowTypeExportError::Io(e)),
            },
            Some("toml") => match File::create(path) {
                Ok(mut file) => {
                    let content = toml::to_string_pretty(&flow_type).map_err(|e| FlowTypeExportError::Serialization(SerializationError::Toml(e)))?;
                    file.write_all(content.as_bytes()).map_err(FlowTypeExportError::Io)
                }
                Err(e) => Err(FlowTypeExportError::Io(e)),
            },
            Some(ext) => Err(FlowTypeExportError::UnsupportedFormat(ext.to_string())),
            None => Err(FlowTypeExportError::UnsupportedFormat(Default::default())),
        }
    }
}
