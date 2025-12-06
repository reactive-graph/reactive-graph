use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;
use uuid::Uuid;

use reactive_graph_graph::EntityInstance;
use reactive_graph_instance_system_api::EntityInstanceExportError;
use reactive_graph_instance_system_api::EntityInstanceImportError;
use reactive_graph_instance_system_api::EntityInstanceImportExportManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;

#[derive(Component)]
pub struct EntityInstanceImportExportManagerImpl {
    reactive_entity_manager: Arc<dyn ReactiveEntityManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl EntityInstanceImportExportManager for EntityInstanceImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<ReactiveEntity, EntityInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let entity_instance: EntityInstance = serde_json::from_reader(reader)?;
        if self.reactive_entity_manager.has(entity_instance.id) {
            return Err(EntityInstanceImportError::EntityAlreadyExists(entity_instance.id));
        }
        self.reactive_entity_manager
            .create_reactive_instance(entity_instance)
            .map_err(EntityInstanceImportError::ReactiveEntityCreationError)
    }

    async fn export(&self, id: Uuid, path: &str) -> Result<(), EntityInstanceExportError> {
        let Some(entity_instance) = self.reactive_entity_manager.get(id) else {
            return Err(EntityInstanceExportError::EntityNotFound(id));
        };
        let Ok(file) = File::create(path) else {
            return Err(EntityInstanceExportError::Io(String::from(path)));
        };
        let entity_instance = EntityInstance::from(entity_instance);
        serde_json::to_writer_pretty(&file, &entity_instance).map_err(EntityInstanceExportError::Serialization)
    }
}

#[async_trait]
impl Lifecycle for EntityInstanceImportExportManagerImpl {}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::InstanceSystemImpl;
    use reactive_graph_graph::EntityInstance;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_instance_system_api::InstanceSystem;
    use reactive_graph_utils_test::DefaultTryFrom;

    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_type_system_impl::TypeSystemSystemImpl;
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_reactive_service_impl::ReactiveSystemImpl;
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_behaviour_service_impl::BehaviourSystemImpl;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_entity_instance_import_export_manager() {
        reactive_graph_utils_test::init_logger();
        let instance_system = reactive_graph_di::get_container::<InstanceSystemImpl>();
        let reactive_system = instance_system.reactive_system();
        let type_system = reactive_system.type_system_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();
        let entity_instance_import_export_manager = instance_system.get_entity_instance_import_export_manager();

        let entity_type = EntityType::random_type().unwrap();
        let entity_type = entity_type_manager.register(entity_type).expect("Failed to register entity type");

        let mut path = env::temp_dir();
        path.push(format!("{}.json", entity_type.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        let entity_instance = EntityInstance::default_try_from(&entity_type).expect("Failed to create random entity instance from an entity type.");

        let reactive_entity = reactive_entity_manager
            .create_reactive_instance(entity_instance.clone())
            .expect("Failed to create reactive instance");
        let uuid = reactive_entity.id;

        entity_instance_import_export_manager
            .export(uuid, &path)
            .await
            .expect("Failed to export entity instance");
        assert!(reactive_entity_manager.has(uuid), "Missing reactive entity with uuid!");
        assert!(reactive_entity_manager.delete(uuid), "Failed to delete reactive entity!");
        assert!(!reactive_entity_manager.has(uuid), "Reactive entity should have been deleted!");
        let imported_reactive_entity = entity_instance_import_export_manager
            .import(&path)
            .await
            .expect("Failed to import entity instance");
        assert_eq!(uuid, imported_reactive_entity.id, "The imported reactive entity's id doesn't match");
        assert!(reactive_entity_manager.has(uuid), "The reactive entity should have been registered during import");
    }
}
