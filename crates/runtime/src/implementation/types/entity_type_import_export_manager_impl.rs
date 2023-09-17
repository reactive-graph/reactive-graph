use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;

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
    fn import(&self, path: &str) -> Result<EntityType, EntityTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_type: EntityType = serde_json::from_reader(reader)?;
        self.entity_type_manager.register(entity_type).map_err(EntityTypeImportError::RegistrationError)
    }

    fn export(&self, ty: &EntityTypeId, path: &str) -> Result<(), EntityTypeExportError> {
        let Some(entity_type) = self.entity_type_manager.get(ty) else {
            return Err(EntityTypeExportError::EntityTypeNotFound(ty.clone()));
        };
        match File::create(path) {
            Ok(file) => serde_json::to_writer_pretty(&file, &entity_type).map_err(EntityTypeExportError::Serialization),
            Err(e) => Err(EntityTypeExportError::Io(e)),
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

    #[test]
    fn test_export_import_entity_type() {
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
            .expect("Failed to export the entity type!");
        assert!(entity_type_manager.has(&entity_ty), "The entity type should be registered!");
        entity_type_manager.delete(&entity_ty).expect("Failed to delete the entity type!");
        assert!(!entity_type_manager.has(&entity_ty), "The entity type shouldn't be registered anymore!");
        entity_type_import_export_manager
            .import(path.as_str())
            .expect("Failed to import the entity type!");
        assert!(entity_type_manager.has(&entity_ty), "The entity type should be registered again!");

        // let entity_type_imported = entity_type_manager.get(&entity_ty).unwrap();
        // println!("{}", serde_json::to_string_pretty(&entity_type_imported).unwrap());
        assert_eq!(
            entity_type_orig,
            entity_type_manager.get(&entity_ty).unwrap(),
            "The imported entity type should match with the constructed entity type!"
        );
    }
}
