use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;
use uuid::Uuid;

use crate::api::EntityInstanceImportExportManager;
use crate::api::ReactiveEntityManager;
use crate::di::component;
use crate::di::Component;
use crate::di::provides;
use crate::di::Wrc;
use crate::error::instances::entity::{EntityInstanceExportError, EntityInstanceImportError};
use crate::model::EntityInstance;
use crate::reactive::ReactiveEntity;

#[component]
pub struct EntityInstanceImportExportManagerImpl {
    reactive_entity_manager: Wrc<dyn ReactiveEntityManager>,
}

#[async_trait]
#[provides]
impl EntityInstanceImportExportManager for EntityInstanceImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<ReactiveEntity, EntityInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_instance: EntityInstance = serde_json::from_reader(reader)?;
        if self.reactive_entity_manager.has(entity_instance.id) {
            return Err(EntityInstanceImportError::EntityAlreadyExists(entity_instance.id));
        }
        self.reactive_entity_manager.create_reactive_instance(entity_instance).map_err(EntityInstanceImportError::ReactiveEntityCreationError)
    }

    async fn export(&self, id: Uuid, path: &str) -> Result<(), EntityInstanceExportError> {
        let Some(entity_instance) = self.reactive_entity_manager.get(id) else {
            return Err(EntityInstanceExportError::EntityNotFound(id));
        };
        let Ok(file) = File::create(path) else {
            return Err(EntityInstanceExportError::Io(String::from(path)));
        };
        let entity_instance = EntityInstance::from(entity_instance);
        serde_json::to_writer_pretty(&file, &entity_instance).map_err(|e| EntityInstanceExportError::Serialization(e))
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::EntityInstance;
    use crate::model::EntityType;
    use crate::model::NamespacedTypeGetter;
    use crate::test_utils::DefaultFrom;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_entity_instance_import_export_manager() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let reactive_entity_manager = runtime.get_reactive_entity_manager();
        let entity_instance_import_export_manager = runtime.get_entity_instance_import_export_manager();

        let entity_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register entity type");

        let mut path = env::temp_dir();
        path.push(format!("{}.json", entity_type.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        let entity_instance = EntityInstance::default_from(&entity_type);

        let reactive_entity = reactive_entity_manager.create_reactive_instance(entity_instance.clone()).expect("Failed to create reactive instance");
        let uuid = reactive_entity.id;

        entity_instance_import_export_manager.export(uuid, &path).await.expect("Failed to export entity instance");
        assert!(reactive_entity_manager.has(uuid), "Missing reactive entity with uuid!");
        assert!(reactive_entity_manager.delete(uuid), "Failed to delete reactive entity!");
        assert!(!reactive_entity_manager.has(uuid), "Reactive entity should have been deleted!");
        let imported_reactive_entity = entity_instance_import_export_manager.import(&path).await.expect("Failed to import entity instance");
        assert_eq!(uuid, imported_reactive_entity.id, "The imported reactive entity's id doesn't match");
        assert!(reactive_entity_manager.has(uuid), "The reactive entity should have been registered during import");
    }
}