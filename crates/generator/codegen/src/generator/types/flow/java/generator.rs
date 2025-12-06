use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::generator::types::CodeGenerationResult;
use crate::generator::types::TypeGenerator;
use crate::targets::CodeGenerationTarget;
use crate::targets::java::Java;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

impl TypeGenerator<Java> for FlowType {
    fn generate_type(&self, config: &CodeGenerationConfig, _resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Java>::absolute_target_path(self, config)?;
        // TODO: Implement java type
        Ok(CodeGenerationResult { path })
    }

    fn relative_target_path(&self) -> Result<PathBuf, CodeGenerationError> {
        let path = self.namespace().relative_path();
        Ok(path.with_extension(Java::extension()))
    }
}
