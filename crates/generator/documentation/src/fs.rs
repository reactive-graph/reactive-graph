use crate::error::DocumentationGenerationError;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinition;
use std::path::PathBuf;
use workspace_root::get_workspace_root;

pub fn create_and_get_documentation_path<T: Into<TypeDefinition>>(ty: T) -> Result<PathBuf, DocumentationGenerationError> {
    let type_definition = ty.into();
    let parent_dir = get_workspace_root()
        .join("docs")
        .join("types")
        .join(type_definition.type_id_type.relative_path())
        .join(type_definition.path().relative_path());
    if !parent_dir.exists() {
        std::fs::create_dir_all(parent_dir.clone()).map_err(|_| DocumentationGenerationError::PathError(parent_dir.clone()))?;
    }
    Ok(parent_dir.join(format!("{}.md", type_definition.namespaced_type.type_name)))
}
