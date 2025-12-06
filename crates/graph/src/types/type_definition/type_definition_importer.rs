use crate::NamespacedTypeContainer;
use crate::TypeDefinitionImportError;
use std::fs::read_dir;
use std::path::PathBuf;

pub trait TypeDefinitionImporter<TY> {
    fn import(path: PathBuf) -> Result<TY, TypeDefinitionImportError>;
}

impl<TYS> TypeDefinitionImporter<TYS> for TYS
where
    TYS: NamespacedTypeContainer,
    TYS::Type: TypeDefinitionImporter<TYS::Type>,
{
    fn import(path: PathBuf) -> Result<Self, TypeDefinitionImportError> {
        if !path.exists() || !path.is_dir() {
            return Err(TypeDefinitionImportError::ImportPathDoesNotExist(path));
        }
        let dir = read_dir(&path)?;
        let tys: TYS = NamespacedTypeContainer::new();
        for entry in dir {
            let path = entry?.path();
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str())
                && file_name.starts_with(".")
            {
                continue;
            }
            let ty: TYS::Type = TYS::Type::import(path)?;
            tys.push(ty);
        }
        Ok(tys)
    }
}
