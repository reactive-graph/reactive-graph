use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;

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
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let component: crate::model::Component = serde_json::from_reader(reader)?;
        self.component_manager.register(component).map_err(ComponentImportError::RegistrationError)
    }

    async fn export(&self, ty: &ComponentTypeId, path: &str) -> Result<(), ComponentExportError> {
        let Some(component) = self.component_manager.get(ty) else {
            return Err(ComponentExportError::ComponentNotFound(ty.clone()));
        };
        match File::create(path) {
            Ok(file) => serde_json::to_writer_pretty(&file, &component).map_err(ComponentExportError::Serialization),
            Err(e) => Err(ComponentExportError::Io(e)),
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

    #[test]
    fn test_export_import_component() {
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
            .expect("Failed to export component");
        assert!(component_manager.has(&component_ty), "Component should be registered!");
        assert!(component_manager.delete(&component_ty), "Failed to delete component!");
        assert!(!component_manager.has(&component_ty), "Component shouldn't be registered anymore!");
        let _component = component_import_export_manager.import(path.as_str()).expect("Failed to import component");
        assert!(component_manager.has(&component_ty), "Component not registered!");
        assert_eq!(
            component_orig,
            component_manager.get(&component_ty).unwrap(),
            "The imported component should match the constructed component!"
        );
    }
}
