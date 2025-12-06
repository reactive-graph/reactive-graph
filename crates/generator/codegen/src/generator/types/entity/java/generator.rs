use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::generator::types::CodeGenerationResult;
use crate::generator::types::TypeGenerator;
use crate::targets::CodeGenerationTarget;
use crate::targets::java::Java;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

impl TypeGenerator<Java> for EntityType {
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

#[cfg(test)]
mod tests {
    use crate::CodeGenerationConfig;
    use crate::generator::types::TypeGenerator;
    use crate::targets::java::Java;
    use convert_case::Case::UpperCamel;
    use convert_case::Casing;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::TypeResolver;

    #[test]
    pub fn test_generate_entity_type_java_class() {
        let config = CodeGenerationConfig::with_temp_dir();
        let resolver = TypeResolver::new();
        let entity_type = EntityType::random_type().unwrap();
        println!("fully qualified namespace: {}", entity_type.namespace());
        let target_path = TypeGenerator::<Java>::absolute_target_path(&entity_type, &config).unwrap();
        println!("target path: {target_path:?}");
        let filename = format!("{}.java", entity_type.type_name().to_case(UpperCamel));
        assert!(target_path.ends_with(filename));
        let code_gen_result = TypeGenerator::<Java>::generate_type(&entity_type, &config, &resolver);
        println!("{code_gen_result:?}");
        let code_gen_result = code_gen_result.unwrap();
        println!("resulting path: {:?}", code_gen_result.path);
        assert_eq!(target_path, code_gen_result.path);
        // assert!(target_path.exists());
    }
}
