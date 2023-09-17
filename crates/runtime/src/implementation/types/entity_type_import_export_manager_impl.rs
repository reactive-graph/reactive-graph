use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use inexor_rgf_rt_api::error::types::serde::DeserializationError;
use inexor_rgf_rt_api::error::types::serde::SerializationError;

use crate::api::EntityTypeImportExportManager;
use crate::api::EntityTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::EntityType;
use crate::model::EntityTypeId;
use crate::rt_api::EntityTypeExportError;
use crate::rt_api::EntityTypeImportError;

#[component]
pub struct EntityTypeImportExportManagerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
}

#[async_trait]
#[provides]
impl EntityTypeImportExportManager for EntityTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let entity_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<EntityType>(&content).map_err(|e| DeserializationError::Json(e).into()),
            Some("json5") => json5::from_str::<EntityType>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            Some("toml") => toml::from_str::<EntityType>(&content).map_err(|e| DeserializationError::Toml(e).into()),
            Some(ext) => Err(EntityTypeImportError::UnsupportedFormat(ext.to_string())),
            None => Err(EntityTypeImportError::UnsupportedFormat(Default::default())),
        }?;
        self.entity_type_manager.register(entity_type).map_err(EntityTypeImportError::RegistrationError)
    }

    async fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        let Some(entity_type) = self.entity_type_manager.get(ty) else {
            return Err(EntityTypeExportError::EntityTypeNotFound(ty.clone()));
        };
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => match File::create(path) {
                Ok(file) => serde_json::to_writer_pretty(&file, &entity_type).map_err(|e| SerializationError::Json(e).into()),
                Err(e) => Err(EntityTypeExportError::Io(e)),
            },
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&entity_type).map_err(|e| EntityTypeExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(EntityTypeExportError::Io)
                }
                Err(e) => Err(EntityTypeExportError::Io(e)),
            },
            Some("toml") => match File::create(path) {
                Ok(mut file) => {
                    let content = toml::to_string_pretty(&entity_type).map_err(|e| EntityTypeExportError::Serialization(SerializationError::Toml(e)))?;
                    file.write_all(content.as_bytes()).map_err(EntityTypeExportError::Io)
                }
                Err(e) => Err(EntityTypeExportError::Io(e)),
            },
            Some(ext) => Err(EntityTypeExportError::UnsupportedFormat(ext.to_string())),
            None => Err(EntityTypeExportError::UnsupportedFormat(Default::default())),
        }
    }
}

#[cfg(test)]
mod test {
    use std::env;

    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::EntityType;
    use crate::model::NamespacedTypeGetter;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_import_entity_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let entity_type_import_export_manager = runtime.get_entity_type_import_export_manager();

        let entity_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register entity type!");
        let entity_type_orig = entity_type.clone();
        let entity_ty = entity_type.ty.clone();
        // println!("{}", serde_json::to_string_pretty(&entity_type_orig).unwrap());

        let mut path = env::temp_dir();
        path.push(format!("{}__{}.json", entity_ty.namespace(), entity_ty.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        assert!(entity_type_manager.has(&entity_ty), "The entity type must exist in order to export it");
        entity_type_import_export_manager
            .export(&entity_ty, path.as_str())
            .await
            .expect("Failed to export the entity type!");
        assert!(entity_type_manager.has(&entity_ty), "The entity type should be registered!");
        entity_type_manager.delete(&entity_ty).expect("Failed to delete the entity type!");
        assert!(!entity_type_manager.has(&entity_ty), "The entity type shouldn't be registered anymore!");
        entity_type_import_export_manager
            .import(path.as_str())
            .await
            .expect("Failed to import the entity type!");
        assert!(entity_type_manager.has(&entity_ty), "The entity type should be registered again!");

        assert_eq!(
            entity_type_orig,
            entity_type_manager.get(&entity_ty).unwrap(),
            "The imported entity type should match with the constructed entity type!"
        );
    }
}
