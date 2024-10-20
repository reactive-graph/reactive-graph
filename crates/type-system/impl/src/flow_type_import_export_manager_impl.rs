use std::fs::File;
use std::io::BufReader;
use std::io::Read;
#[allow(unused_imports)]
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::component_alias;
use springtime_di::Component;

use reactive_graph_graph::FlowType;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_serde::error::DeserializationError;
use reactive_graph_serde::error::SerializationError;
use reactive_graph_type_system_api::FlowTypeExportError;
use reactive_graph_type_system_api::FlowTypeImportError;
use reactive_graph_type_system_api::FlowTypeImportExportManager;
use reactive_graph_type_system_api::FlowTypeManager;

#[derive(Component)]
pub struct FlowTypeImportExportManagerImpl {
    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl FlowTypeImportExportManager for FlowTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<FlowType, FlowTypeImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let flow_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<FlowType>(&content).map_err(|e| DeserializationError::Json(e).into()),
            #[cfg(feature = "json5")]
            Some("json5") => json5::from_str::<FlowType>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            #[cfg(feature = "toml")]
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
            #[cfg(feature = "json5")]
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&flow_type).map_err(|e| FlowTypeExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(FlowTypeExportError::Io)
                }
                Err(e) => Err(FlowTypeExportError::Io(e)),
            },
            #[cfg(feature = "toml")]
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

#[async_trait]
impl Lifecycle for FlowTypeImportExportManagerImpl {}
