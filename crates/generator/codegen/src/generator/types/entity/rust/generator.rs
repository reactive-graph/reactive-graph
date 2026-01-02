use super::const_type_definition::generate_const_type_definition;
use super::struct_definition::generate_struct_definition;
use crate::CodeFormatter;
use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::component::rust::generate_const_components;
use crate::component::rust::generate_impl_component_traits;
use crate::extension::rust::const_extensions::generate_const_extensions;
use crate::generator::types::CodeGenerationResult;
use crate::generator::types::TypeGenerator;
use crate::header::rust::generate_header_generated_code;
use crate::namespace::rust::generate_const_namespace;
use crate::property::rust::generate_const_properties;
use crate::targets::CodeGenerationTarget;
use crate::targets::rust::Rust;
use crate::type_definition::rust::generate_const_type_id;
use convert_case::Case::Snake;
use convert_case::Casing;
use quote::quote;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

impl TypeGenerator<Rust> for EntityType {
    fn generate_type(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        let namespace = self.namespace();
        let Some(parent_namespace) = namespace.parent() else {
            return Err(CodeGenerationError::ParentNamespaceError(namespace));
        };
        parent_namespace.generate_type(config, resolver)?;
        parent_namespace.register_child(config, resolver, &namespace)?;

        println!("Generating source code for entity type {} to {path:?}", self.namespace());

        let properties = resolver.resolve_properties_sorted(self)?;

        let header_generated_code = generate_header_generated_code();
        let const_namespace = generate_const_namespace(self);
        let const_type_id = generate_const_type_id(self);
        let const_properties = generate_const_properties(self, resolver, &properties)?;
        let const_components = generate_const_components(self, resolver)?;
        let const_extensions = generate_const_extensions(self, resolver);
        let const_type_definition = generate_const_type_definition(self)?;
        let struct_definition = generate_struct_definition(self, config, resolver, &properties);
        let impl_component_traits = generate_impl_component_traits(self, resolver)?;

        let token_stream = quote! {
            #header_generated_code
            #const_namespace
            #const_type_id
            #const_properties
            #const_components
            #const_extensions
            #const_type_definition
            #struct_definition
            #impl_component_traits
        };

        let output = Rust::format(token_stream.to_string(), config).map_err(|e| CodeGenerationError::FormatterError(path.clone(), e))?;
        std::fs::write(&path, output).map_err(|e| CodeGenerationError::WriteError(path.clone(), e))?;
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

#[cfg(test)]
mod tests {
    use crate::CodeGenerationConfig;
    use crate::generator::types::TypeGenerator;
    use crate::generator::types::TypeGeneratorForTarget;
    use crate::targets::CodeGenerationTargets;
    use crate::targets::rust::Rust;
    use convert_case::Case::Snake;
    use convert_case::Casing;
    use reactive_graph_graph::EntityType;
    use reactive_graph_graph::NamespacedTypeGetter;
    use reactive_graph_graph::RandomNamespacedType;
    use reactive_graph_graph::TypeResolver;

    #[test]
    pub fn test_type_generator_entity_type_rust_struct() {
        let entity_type = EntityType::random_type().unwrap();
        let resolver = TypeResolver::new();

        let config = CodeGenerationConfig::with_temp_dir();
        println!("fully qualified namespace: {}", entity_type.namespace());
        let target_path = TypeGenerator::<Rust>::absolute_target_path(&entity_type, &config).unwrap();
        println!("target path: {target_path:?}");
        let filename = format!("{}.rs", entity_type.type_name().to_case(Snake));
        assert!(target_path.ends_with(filename));
        let code_gen_result = TypeGenerator::<Rust>::generate_type(&entity_type, &config, &resolver);
        println!("{code_gen_result:?}");
        let code_gen_result = code_gen_result.unwrap();
        println!("resulting path: {:?}", code_gen_result.path);
        assert_eq!(target_path, code_gen_result.path);
        assert!(target_path.exists());
    }

    #[test]
    pub fn test_type_generator_for_language_generate_entity_type_rust_struct() {
        let config = CodeGenerationConfig::with_temp_dir();
        let resolver = TypeResolver::new();
        let entity_type = EntityType::random_type().unwrap();
        println!("fully qualified namespace: {}", entity_type.namespace());
        let target_path = TypeGenerator::<Rust>::absolute_target_path(&entity_type, &config).unwrap();
        println!("target path: {target_path:?}");
        let filename = format!("{}.rs", entity_type.type_name().to_case(Snake));
        assert!(target_path.ends_with(filename));
        let code_gen_result = entity_type.generate_type_for_target(CodeGenerationTargets::Rust, &config, &resolver);
        println!("{code_gen_result:?}");
        let code_gen_result = code_gen_result.unwrap();
        println!("resulting path: {:?}", code_gen_result.path);
        assert_eq!(target_path, code_gen_result.path);
        assert!(target_path.exists());
    }
}
