use std::fs::File;
use std::io::BufReader;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::RelationInstance;
use reactive_graph_graph::RelationInstanceId;
use reactive_graph_instance_system_api::RelationInstanceExportError;
use reactive_graph_instance_system_api::RelationInstanceImportError;
use reactive_graph_instance_system_api::RelationInstanceImportExportManager;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_reactive_service_api::ReactiveRelationManager;

#[derive(Component)]
pub struct RelationInstanceImportExportManagerImpl {
    // entity_instance_import_export_manager: Arc<dyn EntityInstanceImportExportManager>,
    reactive_relation_manager: Arc<dyn ReactiveRelationManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RelationInstanceImportExportManager for RelationInstanceImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_instance: RelationInstance = serde_json::from_reader(reader)?;
        let id = relation_instance.id();
        if self.reactive_relation_manager.has(&id) {
            return Err(RelationInstanceImportError::RelationAlreadyExists(id));
        }
        self.reactive_relation_manager
            .create_reactive_instance(relation_instance)
            .map_err(RelationInstanceImportError::ReactiveRelationCreationError)
    }

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError> {
        let Some(relation_instance) = self.reactive_relation_manager.get(id) else {
            return Err(RelationInstanceExportError::RelationNotFound(id.clone()));
        };
        let Ok(file) = File::create(path) else {
            return Err(RelationInstanceExportError::Io(String::from(path)));
        };
        let relation_instance = RelationInstance::from(relation_instance);
        serde_json::to_writer_pretty(&file, &relation_instance).map_err(RelationInstanceExportError::Serialization)
    }
}

#[async_trait]
impl Lifecycle for RelationInstanceImportExportManagerImpl {}

#[cfg(test)]
mod tests {
    use std::env;

    use default_test::DefaultTest;

    use crate::InstanceSystemImpl;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::NAMESPACE_SEPARATOR;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RelationType;
    use reactive_graph_graph::RelationTypeId;
    use reactive_graph_instance_system_api::InstanceSystem;
    use reactive_graph_reactive_model_api::ReactiveInstance;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use reactive_graph_reactive_model_impl::ReactiveRelation;

    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_type_system_impl::TypeSystemImpl;
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_reactive_service_impl::ReactiveSystemImpl;
    // Do not remove! This import is necessary to make the dependency injection work
    #[allow(unused_imports)]
    use reactive_graph_behaviour_service_impl::BehaviourSystemImpl;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_relation_instance_import_export_manager() {
        let instance_system = reactive_graph_di::get_container::<InstanceSystemImpl>();
        let reactive_system = instance_system.reactive_system();
        let type_system = reactive_system.type_system();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();
        let reactive_entity_manager = reactive_system.get_reactive_entity_manager();
        let reactive_relation_manager = reactive_system.get_reactive_relation_manager();
        let relation_instance_import_export_manager = instance_system.get_relation_instance_import_export_manager();

        // let reactive_relation_manager = runtime.get_reactive_relation_manager();
        // let relation_instance_import_export_manager = runtime.get_relation_instance_import_export_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound entity type");
        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound entity type");

        let relation_ty = RelationTypeId::default_test();
        let relation_type = relation_type_manager
            .register(RelationType::builder_with_ty(&outbound_type, &relation_ty, &inbound_type).build_with_defaults())
            .expect("Failed to register relation type!");
        println!("Registered {relation_ty} -> {}", relation_type.ty);

        let outbound = ReactiveEntity::builder_from_entity_type(&outbound_type).build();
        reactive_entity_manager
            .register_reactive_instance(outbound.clone())
            .expect("Failed to register reactive outbound entity!");
        let inbound = ReactiveEntity::builder_from_entity_type(&inbound_type).build();
        reactive_entity_manager
            .register_reactive_instance(inbound.clone())
            .expect("Failed to register reactive inbound entity!");

        let reactive_relation = ReactiveRelation::builder_from_type_with_unique_id(outbound.clone(), &relation_type, inbound.clone()).build();
        let relation_instance_id = reactive_relation.id();
        // let relation_instance_ty = RelationInstanceTypeId::new_with_random_instance_id(&relation_ty);
        // let relation_instance_id = RelationInstanceId::new_with_random_instance_id(outbound_entity.id, &relation_ty, inbound_entity.id);
        // RelationInstance::builder()
        //     .outbound_id(outbound_entity.id)
        //     .ty(&relation_instance_ty)
        //     .inbound_id(inbound_entity.id)
        //     .properties(PropertyInstances::default_test())
        // let relation_instance = ReactiveRelation::new_from_instance(outbound.clone(), inbound.clone(), );
        reactive_relation_manager
            .register_reactive_instance(reactive_relation.clone())
            .expect("Failed to register reactive relation!");
        println!("Relation instance id {relation_instance_id}");

        let mut path = env::temp_dir();
        path.push(format!(
            "{}--{}{}{}--{}.json",
            relation_instance_id.outbound_id,
            relation_instance_id.namespace(),
            NAMESPACE_SEPARATOR,
            relation_instance_id.type_name(),
            relation_instance_id.inbound_id
        ));
        let path = path.into_os_string().into_string().unwrap();

        relation_instance_import_export_manager
            .export(&relation_instance_id, &path)
            .await
            .expect("Failed to export relation instance");
        assert!(reactive_relation_manager.has(&relation_instance_id), "Reactive relation should still exist!");
        assert!(reactive_relation_manager.delete(&relation_instance_id), "Failed to delete reactive relation!");
        assert!(!reactive_relation_manager.has(&relation_instance_id), "Reactive relation should have been deleted!");
        let imported_reactive_relation = relation_instance_import_export_manager
            .import(&path)
            .await
            .expect("Failed to import relation instance");
        assert_eq!(relation_instance_id, imported_reactive_relation.id(), "The imported reactive relation's id doesn't match");
        assert!(
            reactive_relation_manager.has(&relation_instance_id),
            "The reactive relation should have been registered during import"
        );
    }
}
