use std::fs::File;
use std::io::BufReader;

use async_trait::async_trait;

use crate::api::RelationTypeImportExportManager;
use crate::api::RelationTypeManager;
use crate::di::component;
use crate::di::Component;
use crate::di::provides;
use crate::di::Wrc;
use crate::error::types::relation::RelationTypeExportError;
use crate::error::types::relation::RelationTypeImportError;
use crate::model::RelationType;
use crate::model::RelationTypeId;

#[component]
pub struct RelationTypeImportExportManagerImpl {
    relation_type_manager: Wrc<dyn RelationTypeManager>,
}

#[async_trait]
#[provides]
impl RelationTypeImportExportManager for RelationTypeImportExportManagerImpl {
    fn import(&self, path: &str) -> Result<RelationType, RelationTypeImportError> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let relation_type: RelationType = serde_json::from_reader(reader)?;
        self.relation_type_manager.register(relation_type).map_err(RelationTypeImportError::RegistrationError)
    }

    fn export(&self, ty: &RelationTypeId, path: &str) -> Result<(), RelationTypeExportError> {
        let Some(relation_type) = self.relation_type_manager.get(ty) else {
            return Err(RelationTypeExportError::RelationTypeNotFound(ty.clone()));
        };
        match File::create(path) {
            Ok(file) => serde_json::to_writer_pretty(&file, &relation_type).map_err(RelationTypeExportError::Serialization),
            Err(e) => Err(RelationTypeExportError::Io(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use default_test::DefaultTest;

    use crate::get_runtime;
    use crate::model::EntityType;
    use crate::model::RelationTypeId;
    use crate::model::NamespacedTypeGetter;
    use crate::model::RelationType;

    #[test]
    fn test_export_import_relation_type() {
        let runtime = get_runtime();
        let entity_type_manager = runtime.get_entity_type_manager();
        let relation_type_manager = runtime.get_relation_type_manager();
        let relation_type_import_export_manager = runtime.get_relation_type_import_export_manager();

        let outbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register outbound type");
        let outbound_ty = outbound_type.ty.clone();

        let inbound_type = entity_type_manager.register(EntityType::default_test()).expect("Failed to register inbound type");
        let inbound_ty = inbound_type.ty.clone();

        let relation_ty = RelationTypeId::default_test();

        let mut path = env::temp_dir();
        path.push(format!("{}__{}.json", relation_ty.namespace(), relation_ty.type_name()));
        let path = path.into_os_string().into_string().unwrap();

        let relation_type = relation_type_manager.register(RelationType::builder_with_ty(outbound_ty, &relation_ty, inbound_ty).build_with_defaults()).expect("Failed to register relation type!");
        let relation_type_orig = relation_type.clone();
        // println!("{}", serde_json::to_string_pretty(&relation_type_orig).unwrap());

        assert!(relation_type_manager.has(&relation_ty), "The relation type must exist in order to export it");
        relation_type_import_export_manager.export(&relation_ty, path.as_str()).expect("Failed to export the relation type!");
        assert!(relation_type_manager.has(&relation_ty), "The relation type should be registered!");
        relation_type_manager.delete(&relation_ty).expect("Failed to delete the relation type!");
        assert!(!relation_type_manager.has(&relation_ty), "The relation type shouldn't be registered anymore!");
        let _relation_type = relation_type_import_export_manager.import(path.as_str()).expect("Failed to import the relation type!");
        assert!(relation_type_manager.has(&relation_ty), "The relation type should be registered again!");

        // let relation_type_imported = relation_type_manager.get(&relation_ty).unwrap();
        // println!("{}", serde_json::to_string_pretty(&relation_type_imported).unwrap());
        assert_eq!(relation_type_orig, relation_type_manager.get(&relation_ty).unwrap(), "The imported relation type should match with the constructed relation type!");
    }
}