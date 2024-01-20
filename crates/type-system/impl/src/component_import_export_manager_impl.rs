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

use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_type_system_api::ComponentExportError;
use inexor_rgf_type_system_api::ComponentImportError;
use inexor_rgf_type_system_api::ComponentImportExportManager;
use inexor_rgf_type_system_api::ComponentManager;
use inexor_rgf_type_system_api::DeserializationError;
use inexor_rgf_type_system_api::SerializationError;

#[derive(Component)]
pub struct ComponentImportExportManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ComponentImportExportManager for ComponentImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<inexor_rgf_graph::Component, ComponentImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let component = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<inexor_rgf_graph::Component>(&content).map_err(|e| DeserializationError::Json(e).into()),
            #[cfg(json5)]
            Some("json5") => json5::from_str::<inexor_rgf_graph::Component>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            #[cfg(toml)]
            Some("toml") => toml::from_str::<inexor_rgf_graph::Component>(&content).map_err(|e| DeserializationError::Toml(e).into()),
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
            #[cfg(json5)]
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&component).map_err(|e| ComponentExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(ComponentExportError::Io)
                }
                Err(e) => Err(ComponentExportError::Io(e)),
            },
            #[cfg(toml)]
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

#[async_trait]
impl Lifecycle for ComponentImportExportManagerImpl {}

#[cfg(test)]
mod test {
    extern crate test;

    use std::env;

    use default_test::DefaultTest;

    use crate::TypeSystemImpl;
    use inexor_rgf_graph::Component;
    use inexor_rgf_graph::NamespacedTypeGetter;
    use inexor_rgf_type_system_api::TypeSystem;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_import_component() {
        inexor_rgf_test_utils::init_logger();
        let type_system = inexor_rgf_di::get_container::<TypeSystemImpl>();
        let component_manager = type_system.get_component_manager();
        let component_import_export_manager = type_system.get_component_import_export_manager();

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
