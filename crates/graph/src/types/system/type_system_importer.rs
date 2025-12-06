use crate::Components;
use crate::EntityTypes;
use crate::FlowTypes;
use crate::RelationTypes;
use crate::TypeDefinitionImportError;
use crate::TypeDefinitionImporter;
use crate::TypeSystem;
use std::path::PathBuf;

impl TypeDefinitionImporter<TypeSystem> for TypeSystem {
    fn import(path: PathBuf) -> Result<Self, TypeDefinitionImportError> {
        if !path.exists() || !path.is_dir() {
            return Err(TypeDefinitionImportError::ImportPathDoesNotExist(path));
        }
        Ok(TypeSystem::builder()
            .components(Components::import(path.join("components")).unwrap_or_default())
            .entity_types(EntityTypes::import(path.join("entities")).unwrap_or_default())
            .relation_types(RelationTypes::import(path.join("relations")).unwrap_or_default())
            .flow_types(FlowTypes::import(path.join("flows")).unwrap_or_default())
            .build())
    }
}
