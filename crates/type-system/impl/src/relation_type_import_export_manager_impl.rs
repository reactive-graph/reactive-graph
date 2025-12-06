use std::fs::File;
use std::io::BufReader;
use std::io::Read;
#[allow(unused_imports)]
use std::io::Write;
use std::path::Path;
use std::sync::Arc;

use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_graph::RelationType;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_serde::error::DeserializationError;
use reactive_graph_serde::error::SerializationError;
use reactive_graph_type_system_api::RelationTypeExportError;
use reactive_graph_type_system_api::RelationTypeImportError;
use reactive_graph_type_system_api::RelationTypeImportExportManager;
use reactive_graph_type_system_api::RelationTypeManager;

#[derive(Component)]
pub struct RelationTypeImportExportManagerImpl {
    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl RelationTypeImportExportManager for RelationTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let relation_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<RelationType>(&content).map_err(|e| DeserializationError::Json(e).into()),
            #[cfg(feature = "json5")]
            Some("json5") => json5::from_str::<RelationType>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            #[cfg(feature = "toml")]
            Some("toml") => toml::from_str::<RelationType>(&content).map_err(|e| DeserializationError::Toml(e).into()),
            Some(ext) => Err(RelationTypeImportError::UnsupportedFormat(ext.to_string())),
            None => Err(RelationTypeImportError::UnsupportedFormat(Default::default())),
        }?;
        self.relation_type_manager
            .register(relation_type)
            .map_err(RelationTypeImportError::RegistrationError)
    }

    async fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError> {
        let Some(relation_type) = self.relation_type_manager.get(ty) else {
            return Err(RelationTypeExportError::RelationTypeNotFound(ty.clone()));
        };
        let path = Path::new(path);
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => match File::create(path) {
                Ok(file) => serde_json::to_writer_pretty(&file, &relation_type).map_err(|e| SerializationError::Json(e).into()),
                Err(e) => Err(RelationTypeExportError::Io(e)),
            },
            #[cfg(feature = "json5")]
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&relation_type).map_err(|e| RelationTypeExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(RelationTypeExportError::Io)
                }
                Err(e) => Err(RelationTypeExportError::Io(e)),
            },
            #[cfg(feature = "toml")]
            Some("toml") => match File::create(path) {
                Ok(mut file) => {
                    let content = toml::to_string_pretty(&relation_type).map_err(|e| RelationTypeExportError::Serialization(SerializationError::Toml(e)))?;
                    file.write_all(content.as_bytes()).map_err(RelationTypeExportError::Io)
                }
                Err(e) => Err(RelationTypeExportError::Io(e)),
            },
            Some(ext) => Err(RelationTypeExportError::UnsupportedFormat(ext.to_string())),
            None => Err(RelationTypeExportError::UnsupportedFormat(Default::default())),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use crate::TypeSystemSystemImpl;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::RandomNamespacedTypeId;
    use reactive_graph_graph::RelationType;
    use reactive_graph_graph::RelationTypeId;
    use reactive_graph_type_system_api::TypeSystemSystem;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_import_relation_type() {
        reactive_graph_utils_test::init_logger();
        let type_system = reactive_graph_di::get_container::<TypeSystemSystemImpl>();
        let entity_type_manager = type_system.get_entity_type_manager();
        let relation_type_manager = type_system.get_relation_type_manager();
        let relation_type_import_export_manager = type_system.get_relation_type_import_export_manager();

        let outbound_entity_type = EntityType::random_type().unwrap();
        let outbound_type = entity_type_manager.register(outbound_entity_type).expect("Failed to register outbound type");
        let outbound_ty = outbound_type.ty.clone();

        let inbound_entity_type = EntityType::random_type().unwrap();
        let inbound_type = entity_type_manager.register(inbound_entity_type).expect("Failed to register inbound type");
        let inbound_ty = inbound_type.ty.clone();

        let relation_ty = RelationTypeId::random_type_id().unwrap();

        let mut path = env::temp_dir();
        path.push(format!("{}__{}.json", relation_ty.namespace(), relation_ty.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        let relation_type = RelationType::builder_with_ty(outbound_ty, &relation_ty, inbound_ty)
            .build_with_defaults()
            .expect("Failed to construct a random relation type");
        let relation_type = relation_type_manager.register(relation_type).expect("Failed to register relation type!");
        let relation_type_orig = relation_type.clone();

        assert!(relation_type_manager.has(&relation_ty), "The relation type must exist in order to export it");
        relation_type_import_export_manager
            .export(&relation_ty, path.as_str())
            .await
            .expect("Failed to export the relation type!");
        assert!(relation_type_manager.has(&relation_ty), "The relation type should be registered!");
        relation_type_manager.delete(&relation_ty).expect("Failed to delete the relation type!");
        assert!(!relation_type_manager.has(&relation_ty), "The relation type shouldn't be registered anymore!");
        let _relation_type = relation_type_import_export_manager
            .import(path.as_str())
            .await
            .expect("Failed to import the relation type!");
        assert!(relation_type_manager.has(&relation_ty), "The relation type should be registered again!");

        assert_eq!(
            relation_type_orig,
            relation_type_manager.get(&relation_ty).unwrap(),
            "The imported relation type should match with the constructed relation type!"
        );
    }
}

#[async_trait]
impl Lifecycle for RelationTypeImportExportManagerImpl {}
