use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::generator::types::result::CodeGenerationResult;
use crate::targets::CodeGenerationTarget;
use crate::targets::CodeGenerationTargets;
use crate::targets::java::Java;
use crate::targets::rust::Rust;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

pub trait TypeGeneratorForTarget: TypeGenerator<Java> + TypeGenerator<Rust> {
    fn generate_type_for_target(
        &self,
        target: CodeGenerationTargets,
        config: &CodeGenerationConfig,
        resolver: &TypeResolver,
    ) -> Result<CodeGenerationResult, CodeGenerationError> {
        match target {
            CodeGenerationTargets::Java => TypeGenerator::<Java>::generate_type(self, config, resolver),
            CodeGenerationTargets::Rust => TypeGenerator::<Rust>::generate_type(self, config, resolver),
            _ => Err(CodeGenerationError::TargetNotSupported(target)),
        }
    }
}

impl<TY> TypeGeneratorForTarget for TY where TY: TypeGenerator<Java> + TypeGenerator<Rust> {}

pub trait TypeGenerator<Target: CodeGenerationTarget> {
    fn generate_type(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError>;

    fn register_child(
        &self,
        config: &CodeGenerationConfig,
        _resolver: &TypeResolver,
        _child_namespace: &Namespace,
    ) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = self.absolute_target_path(config)?;
        Ok(CodeGenerationResult { path })
    }

    fn register_root(&self, config: &CodeGenerationConfig, _resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        Ok(CodeGenerationResult { path: config.base_path() })
    }

    fn absolute_target_path(&self, config: &CodeGenerationConfig) -> Result<PathBuf, CodeGenerationError> {
        Ok(config.as_ref().join(self.relative_target_path()?))
    }
    fn relative_target_path(&self) -> Result<PathBuf, CodeGenerationError>;
}
