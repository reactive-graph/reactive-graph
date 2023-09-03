use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;

use crate::api::ReactiveRelationManager;
use crate::api::RelationInstanceImportExportManager;
use crate::di::*;
use crate::error::instances::relation::RelationInstanceExportError;
use crate::error::instances::relation::RelationInstanceImportError;
use crate::model::RelationInstance;
use crate::model::RelationInstanceId;
use crate::reactive::ReactiveRelation;

#[component]
pub struct RelationInstanceImportExportManagerImpl {
    // entity_instance_import_export_manager: Wrc<dyn EntityInstanceImportExportManager>,
    reactive_relation_manager: Wrc<dyn ReactiveRelationManager>,
}

#[async_trait]
#[provides]
impl RelationInstanceImportExportManager for RelationInstanceImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<ReactiveRelation, RelationInstanceImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_instance: RelationInstance = serde_json::from_reader(reader)?;
        let id = relation_instance.id();
        if self.reactive_relation_manager.has(&id) {
            return Err(RelationInstanceImportError::RelationAlreadyExists(id));
        }
        self.reactive_relation_manager.create_reactive_instance(relation_instance).map_err(RelationInstanceImportError::ReactiveRelationCreationError)
    }

    async fn export(&self, id: &RelationInstanceId, path: &str) -> Result<(), RelationInstanceExportError> {
        let Some(relation_instance) = self.reactive_relation_manager.get(id) else {
            return Err(RelationInstanceExportError::RelationNotFound(id.clone()));
        };
        let Ok(file) = File::create(path) else {
            return Err(RelationInstanceExportError::Io(String::from(path)));
        };
        let relation_instance = RelationInstance::from(relation_instance);
        serde_json::to_writer_pretty(&file, &relation_instance).map_err(|e| RelationInstanceExportError::Serialization(e))
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::EntityType;
    use crate::model::NamespacedTypeGetter;
    use crate::model::RelationType;
    use crate::model::RelationTypeId;
    use crate::reactive::ReactiveEntity;
    use crate::reactive::ReactiveInstance;
    use crate::reactive::ReactiveRelation;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_relation_instance_import_export_manager() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();
        let reactive_entity_manager = runtime.get_reactive_entity_manager();
        let reactive_relation_manager = runtime.get_reactive_relation_manager();
        let relation_instance_import_export_manager = runtime.get_relation_instance_import_export_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound entity type");
        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound entity type");

        let relation_ty = RelationTypeId::default_test();
        let relation_type = relation_type_manager
            .register(RelationType::builder_with_ty(&outbound_type, &relation_ty, &inbound_type).build_with_defaults())
            .expect("Failed to register relation type!");
        println!("Registered {relation_ty} -> {}", relation_type.ty);

        let outbound = ReactiveEntity::builder_from_entity_type(&outbound_type).build();
        reactive_entity_manager.register_reactive_instance(outbound.clone()).expect("Failed to register reactive outbound entity!");
        let inbound = ReactiveEntity::builder_from_entity_type(&inbound_type).build();
        reactive_entity_manager.register_reactive_instance(inbound.clone()).expect("Failed to register reactive inbound entity!");

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
        reactive_relation_manager.register_reactive_instance(reactive_relation.clone()).expect("Failed to register reactive relation!");
        println!("Relation instance id {relation_instance_id}");

        let mut path = env::temp_dir();
        path.push(format!("{}--{}__{}--{}.json", relation_instance_id.outbound_id, relation_instance_id.namespace(), relation_instance_id.type_name(), relation_instance_id.inbound_id));
        let path = path.into_os_string().into_string().unwrap();

        relation_instance_import_export_manager.export(&relation_instance_id, &path).await.expect("Failed to export relation instance");
        assert!(reactive_relation_manager.has(&relation_instance_id), "Reactive relation should still exist!");
        assert!(reactive_relation_manager.delete(&relation_instance_id), "Failed to delete reactive relation!");
        assert!(!reactive_relation_manager.has(&relation_instance_id), "Reactive relation should have been deleted!");
        let imported_reactive_relation = relation_instance_import_export_manager.import(&path).await.expect("Failed to import relation instance");
        assert_eq!(relation_instance_id, imported_reactive_relation.id(), "The imported reactive relation's id doesn't match");
        assert!(reactive_relation_manager.has(&relation_instance_id), "The reactive relation should have been registered during import");
    }

}