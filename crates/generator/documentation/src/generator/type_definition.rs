use crate::error::DocumentationGenerationError;
use crate::fs::create_and_get_documentation_path;
use crate::generator::TypedMarkdownDocumentation;
use crate::types::config::DocumentationConfig;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeSystem;
use std::fs::write;
use std::path::PathBuf;

pub trait MarkdownDocumentationWriter {
    fn write(&self, base_path: &PathBuf) -> Result<PathBuf, DocumentationGenerationError>;
    fn write_to(&self, path: PathBuf) -> Result<PathBuf, DocumentationGenerationError>;
}

impl<TY> TypedMarkdownDocumentation<TY>
where
    TY: TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    pub fn ty(self, config: &DocumentationConfig) -> Self {
        let type_definition = self.ty.type_definition();
        {
            let mut document = self.document.write().unwrap();
            if config.header {
                document.header1(format!("{} `{}`", type_definition.type_id_type.full_name(), type_definition.type_name()));
                document.header2("Fully Qualified Namespace");
            }
            document.paragraph(format!("`{}`", type_definition.namespace()));
        }
        self
    }
}

impl<TY> MarkdownDocumentationWriter for TypedMarkdownDocumentation<TY>
where
    TY: TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    fn write(&self, base_path: &PathBuf) -> Result<PathBuf, DocumentationGenerationError> {
        let path = create_and_get_documentation_path(base_path, self.ty.type_definition())?;
        self.write_to(path)
    }

    fn write_to(&self, path: PathBuf) -> Result<PathBuf, DocumentationGenerationError> {
        match write(path.clone(), self.to_string()) {
            Ok(_) => Ok(path),
            Err(e) => Err(DocumentationGenerationError::WriteError(path, e))?,
        }
    }
}

impl MarkdownDocumentationWriter for TypeSystem {
    fn write(&self, _base_path: &PathBuf) -> Result<PathBuf, DocumentationGenerationError> {
        // TODO: Implement the root README.md which let you navigate to the namespaces.
        todo!()
    }

    fn write_to(&self, _path: PathBuf) -> Result<PathBuf, DocumentationGenerationError> {
        // TODO: Implement the root README.md which let you navigate to the namespaces.
        todo!()
    }
}
