use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use async_trait::async_trait;
use inexor_rgf_rt_api::error::types::serde::DeserializationError;
use inexor_rgf_rt_api::error::types::serde::SerializationError;

use crate::api::RelationTypeImportExportManager;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::provides;
use crate::di::Component;
use crate::di::Wrc;
use crate::model::RelationType;
use crate::model::RelationTypeId;
use crate::rt_api::RelationTypeExportError;
use crate::rt_api::RelationTypeImportError;

#[component]
pub struct RelationTypeImportExportManagerImpl {
    relation_type_manager: Wrc<dyn RelationTypeManager>,
}

#[async_trait]
#[provides]
impl RelationTypeImportExportManager for RelationTypeImportExportManagerImpl {
    async fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        let path = Path::new(path);
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        let relation_type = match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<RelationType>(&content).map_err(|e| DeserializationError::Json(e).into()),
            Some("json5") => json5::from_str::<RelationType>(&content).map_err(|e| DeserializationError::Json5(e).into()),
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
            Some("json5") => match File::create(path) {
                Ok(mut file) => {
                    let content = json5::to_string(&relation_type).map_err(|e| RelationTypeExportError::Serialization(SerializationError::Json5(e)))?;
                    file.write_all(content.as_bytes()).map_err(RelationTypeExportError::Io)
                }
                Err(e) => Err(RelationTypeExportError::Io(e)),
            },
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

    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::EntityType;
    use crate::model::NamespacedTypeGetter;
    use crate::model::RelationType;
    use crate::model::RelationTypeId;

    #[tokio::test(flavor = "multi_thread")]
    async fn test_export_import_relation_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();
        let relation_type_import_export_manager = runtime.get_relation_type_import_export_manager();

        let outbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register outbound type");
        let outbound_ty = outbound_type.ty.clone();

        let inbound_type = entity_type_manager
            .register(EntityType::default_test())
            .expect("Failed to register inbound type");
        let inbound_ty = inbound_type.ty.clone();

        let relation_ty = RelationTypeId::default_test();

        let mut path = env::temp_dir();
        path.push(format!("{}__{}.json", relation_ty.namespace(), relation_ty.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        let relation_type = relation_type_manager
            .register(RelationType::builder_with_ty(outbound_ty, &relation_ty, inbound_ty).build_with_defaults())
            .expect("Failed to register relation type!");
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
