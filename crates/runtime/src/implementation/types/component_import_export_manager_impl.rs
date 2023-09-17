use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use inexor_rgf_rt_api::error::types::serde::DeserializationError;
use inexor_rgf_rt_api::error::types::serde::SerializationError;

use crate::api::ComponentImportExportManager;
use crate::api::ComponentManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::ComponentTypeId;
use crate::rt_api::ComponentExportError;
use crate::rt_api::ComponentImportError;

#[component]
pub struct ComponentImportExportManagerImpl {
    component_manager: Wrc<dyn ComponentManager>,
}

#[async_trait]
#[provides]
impl ComponentImportExportManager for ComponentImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<crate::model::Component, ComponentImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let component = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<crate::model::Component>(&content).map_err(|e| DeserializationError::Json(e).into()),
            Some("json5") => json5::from_str::<crate::model::Component>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            Some("toml") => toml::from_str::<crate::model::Component>(&content).map_err(|e| DeserializationError::Toml(e).into()),
            Some(ext) => Err(ComponentImportError::UnsupportedFormat(ext.to_string())),
            None => Err(ComponentImportError::UnsupportedFormat(Default::default())),
        }?;
        self.component_manager.register(component).map_err(ComponentImportError::RegistrationError)
    }

    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError> {
        let Some(component) = self.component_manager.get(ty) else {
            return Err(ComponentExportError::ComponentNotFound(ty.clone()));
        };
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => match File::create(path) {
                Ok(file) => serde_json::to_writer_pretty(&file, &component).map_err(|e| SerializationError::Json(e).into()),
                Err(e) => Err(ComponentExportError::Io(e)),
            },
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&component).map_err(|e| ComponentExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(ComponentExportError::Io)
                }
                Err(e) => Err(ComponentExportError::Io(e)),
            },
            Some("toml") => match File::create(path) {
                Ok(mut file) => {
                    let content = toml::to_string_pretty(&component).map_err(|e| ComponentExportError::Serialization(SerializationError::Toml(e)))?;
                    file.write_all(content.as_bytes()).map_err(ComponentExportError::Io)
                }
                Err(e) => Err(ComponentExportError::Io(e)),
            },
            Some(ext) => Err(ComponentExportError::UnsupportedFormat(ext.to_string())),
            None => Err(ComponentExportError::UnsupportedFormat(Default::default())),
        }
    }
}

#[cfg(test)]
mod test {
    extern crate test;

    use std::env;

    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::Component;
    use crate::model::NamespacedTypeGetter;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_import_component() {
        let runtime = get_runtime();
        let component_manager = runtime.get_component_manager();
        let component_import_export_manager = runtime.get_component_import_export_manager();

        let component = component_manager.register(Component::default_test()).expect("Failed to create component");
        let component_orig = component.clone();
        let component_ty = component.ty.clone();

        let mut path = env::temp_dir();
        path.push(format!("{}__{}.json", component_ty.namespace(), component_ty.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        component_import_export_manager
            .export(&component_ty, path.as_str())
            .await
            .expect("Failed to export component");
        assert!(component_manager.has(&component_ty), "Component should be registered!");
        assert!(component_manager.delete(&component_ty), "Failed to delete component!");
        assert!(!component_manager.has(&component_ty), "Component shouldn't be registered anymore!");
        let _component = component_import_export_manager.import(path.as_str()).await.expect("Failed to import component");
        assert!(component_manager.has(&component_ty), "Component not registered!");
        assert_eq!(
            component_orig,
            component_manager.get(&component_ty).unwrap(),
            "The imported component should match the constructed component!"
        );
    }
}
