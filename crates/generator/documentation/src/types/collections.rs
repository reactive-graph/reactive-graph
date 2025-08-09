use crate::error::DocumentationGenerationError;
use crate::generator::MarkdownDocumentation;
use crate::types::GenerateDocumentation;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use std::hash::Hash;
use std::path::PathBuf;

pub trait GenerateDocumentations
where
    Self::Container: NamespacedTypeContainer<Type = Self::Type>,
    Self::Type: GenerateDocumentation<Self::Type>,
{
    type Container;
    type Type;

    fn generate_documentations(&self) -> Result<Vec<MarkdownDocumentation<Self::Type>>, DocumentationGenerationError>;

    fn write_documentations(&self) -> Result<Vec<PathBuf>, DocumentationGenerationError>;
}

impl<TYS, TY> GenerateDocumentations for TYS
where
    TYS: NamespacedTypeContainer<Type = TY>,
    TY: GenerateDocumentation<TY> + Eq + Hash + TypeDefinitionGetter + NamespacedTypeGetter + Clone,
{
    type Container = TYS;
    type Type = TY;

    fn generate_documentations(&self) -> Result<Vec<MarkdownDocumentation<Self::Type>>, DocumentationGenerationError> {
        let mut v = vec![];
        for entry in self.types().iter() {
            let ty = entry.key();
            v.push(ty.generate_documentation()?);
        }
        Ok(v)
    }

    fn write_documentations(&self) -> Result<Vec<PathBuf>, DocumentationGenerationError> {
        let mut v = vec![];
        for documentation in self.generate_documentations()? {
            v.push(documentation.write()?);
        }
        Ok(v)
    }
}
