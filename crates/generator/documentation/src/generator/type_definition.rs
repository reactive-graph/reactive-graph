use crate::error::DocumentationGenerationError;
use crate::fs::create_and_get_documentation_path;
use crate::generator::MarkdownDocumentation;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use std::fs::write;
use std::path::PathBuf;

impl<TY> MarkdownDocumentation<TY>
where
    TY: TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    pub fn ty(mut self) -> Self {
        let type_definition = self.ty.type_definition();

        self.document.header1(format!(
            "{} {} {}",
            type_definition.type_id_type.full_name(),
            type_definition.path(),
            type_definition.type_name()
        ));
        self
    }

    pub fn write(&self) -> Result<PathBuf, DocumentationGenerationError> {
        let path = create_and_get_documentation_path(self.ty.type_definition())?;
        self.write_to(path)
    }

    pub fn write_to(&self, path: PathBuf) -> Result<PathBuf, DocumentationGenerationError> {
        match write(path.clone(), self.to_string()) {
            Ok(_) => Ok(path),
            Err(e) => Err(DocumentationGenerationError::WriteError(path, e))?,
        }
        // .map_err(|e| DocumentationGenerationError::WriteError(path, e))
        // .map_err(|_| anyhow!("Failed to write {} to {:?}", self.ty.type_definition(), path))?;
        // Ok(path)
    }
}
