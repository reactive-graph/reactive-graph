use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::CodeGenerationResult;
use crate::CodeGenerationTarget;
use crate::TypeGenerator;
use crate::rust::Rust;
use convert_case::Case::Snake;
use convert_case::Casing;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

impl TypeGenerator<Rust> for FlowType {
    fn generate_type(&self, config: &CodeGenerationConfig, _resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        // TODO:
        Ok(CodeGenerationResult { path })
    }

    fn relative_target_path(&self) -> Result<PathBuf, CodeGenerationError> {
        let namespace = self.namespace();
        Ok(namespace
            .parent()
            .ok_or_else(|| CodeGenerationError::ParentNamespaceError(namespace))?
            .relative_path()
            .join(self.type_name().to_case(Snake))
            .with_extension(Rust::extension()))
    }
}
