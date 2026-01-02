use crate::EntityType;
use crate::TypeDefinitionImportError;
use crate::TypeDefinitionImporter;
use reactive_graph_serde::error::DeserializationError;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::path::PathBuf;

impl TypeDefinitionImporter<EntityType> for EntityType {
    fn import(path: PathBuf) -> Result<Self, TypeDefinitionImportError> {
        if !path.is_file() {
            return Err(TypeDefinitionImportError::DirEntryError(path.clone()));
        }
        let file = File::open(&path)?;
        let mut reader = BufReader::new(file);
        let mut content = String::new();
        reader.read_to_string(&mut content)?;
        match path.extension().and_then(|ext| ext.to_str()) {
            Some("json") => serde_json::from_str::<Self>(&content).map_err(|e| DeserializationError::Json(e).into()),
            #[cfg(feature = "json5")]
            Some("json5") => json5::from_str::<Self>(&content).map_err(|e| DeserializationError::Json5(e).into()),
            #[cfg(feature = "toml")]
            Some("toml") => toml::from_str::<Self>(&content).map_err(|e| DeserializationError::Toml(e).into()),
            Some(ext) => Err(TypeDefinitionImportError::UnsupportedFormat(ext.to_string(), path.clone())),
            None => Err(TypeDefinitionImportError::UnsupportedFormat(Default::default(), path.clone())),
        }
    }
}
