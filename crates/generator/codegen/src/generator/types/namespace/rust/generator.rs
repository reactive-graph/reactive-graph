use crate::CodeFormatter;
use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::generator::types::CodeGenerationResult;
use crate::generator::types::TypeGenerator;
use crate::namespace::rust::item_sort::sort_file_items;
use crate::targets::CodeGenerationTarget;
use crate::targets::rust::Rust;
use convert_case::Case::Snake;
use convert_case::Casing;
use proc_macro2::Span;
use quote::quote;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::TypeResolver;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::Ident;
use syn::parse_quote;

impl TypeGenerator<Rust> for Namespace {
    fn generate_type(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        match self.parent() {
            Some(parent_namespace) => {
                parent_namespace.register_child(config, resolver, self)?;
            }
            None => {
                println!("===> Registering root");
                self.register_root(config, resolver)?;
            }
        }
        if path.exists() {
            return Ok(CodeGenerationResult { path });
        }
        let module_dir_path = path.parent().ok_or_else(|| CodeGenerationError::CreateModuleDirError(path.clone()))?;
        let _ = create_dir_all(module_dir_path);
        println!("Generating source code for namespace {self} in {}", path.display());
        File::create(&path).map_err(|e| CodeGenerationError::CreateModuleError(path.clone(), e))?;
        Ok(CodeGenerationResult { path })
    }

    fn register_child(
        &self,
        config: &CodeGenerationConfig,
        resolver: &TypeResolver,
        child_namespace: &Namespace,
    ) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        // if !path.exists() {
        self.generate_type(config, resolver)?;
        // }
        println!("Registering child {child_namespace} of {self} in {}", path.display());
        let last_segment = child_namespace
            .last_segment()
            .ok_or_else(|| CodeGenerationError::ParentNamespaceError(child_namespace.clone()))?;
        let child_module_name = if child_namespace.is_path() {
            last_segment.to_string()
        } else {
            last_segment.to_string().to_case(Snake)
        };
        println!("Child module name: {child_module_name}");

        // Get existing modules
        let content = std::fs::read_to_string(&path).map_err(|e| CodeGenerationError::ReadError(path.clone(), e))?;
        let mut syntax_tree = syn::parse_file(&content).map_err(|e| CodeGenerationError::ParserError(path.clone(), e))?;
        let mut exists = false;
        for item in syntax_tree.items.iter() {
            if let syn::Item::Mod(item_mod) = item {
                if item_mod.content.is_none() {
                    if item_mod.ident.to_string() == child_module_name {
                        exists = true;
                    }
                }
            }
        }
        if !exists {
            let child_module_ident = Ident::new(&child_module_name, Span::call_site());
            let child_module_item = parse_quote! {
                pub mod #child_module_ident;
            };
            syntax_tree.items.push(child_module_item);

            if child_namespace.is_type() {
                let type_name = child_namespace
                    .last_segment()
                    .ok_or_else(|| CodeGenerationError::ParentNamespaceError(child_namespace.clone()))?
                    .to_string();
                let type_name_ident = Ident::new(&type_name, Span::call_site());
                let type_name_item = parse_quote! {
                    pub use #child_module_ident::#type_name_ident;
                };
                syntax_tree.items.push(type_name_item);
            }
            sort_file_items(&mut syntax_tree);
            let token_stream = quote! { #syntax_tree };
            let formatted = Rust::format(token_stream.to_string(), config).map_err(|e| CodeGenerationError::FormatterError(path.clone(), e))?;
            std::fs::write(&path, formatted).map_err(|e| CodeGenerationError::WriteError(path.clone(), e))?;
        }
        Ok(CodeGenerationResult { path })
    }

    fn register_root(&self, config: &CodeGenerationConfig, _resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        println!("Registering root namespace {self} in {:?}", config.as_ref());
        let root_module_name = self.to_string().to_case(Snake);
        println!("Root module name: {root_module_name}");
        let path = config.base_path().join("lib").with_extension(Rust::extension());
        println!("Root module path: {path:?}");

        let _ = create_dir_all(config);
        if !path.exists() {
            File::create(&path).map_err(|e| CodeGenerationError::CreateModuleError(path.clone(), e))?;
        }

        // Get existing modules
        let content = std::fs::read_to_string(&path).map_err(|e| CodeGenerationError::ReadError(path.clone(), e))?;
        let mut syntax_tree = syn::parse_file(&content).map_err(|e| CodeGenerationError::ParserError(path.clone(), e))?;
        let mut exists = false;
        for item in syntax_tree.items.iter() {
            if let syn::Item::Mod(item_mod) = item {
                if item_mod.content.is_none() {
                    println!("Check if {} == {}", item_mod.ident.to_string(), root_module_name);
                    if item_mod.ident.to_string() == root_module_name {
                        println!("Module {root_module_name} already exists");
                        exists = true;
                    }
                }
            }
        }
        if !exists {
            let root_module_ident = Ident::new(&root_module_name, Span::call_site());
            let root_module_item = parse_quote! {
                pub mod #root_module_ident;
            };
            syntax_tree.items.push(root_module_item);
            sort_file_items(&mut syntax_tree);
            let token_stream = quote! { #syntax_tree };
            let formatted = Rust::format(token_stream.to_string(), config).map_err(|e| CodeGenerationError::FormatterError(path.clone(), e))?;

            std::fs::write(&path, formatted).map_err(|e| CodeGenerationError::WriteError(path.clone(), e))?;
        }
        Ok(CodeGenerationResult { path })
    }

    fn relative_target_path(&self) -> Result<PathBuf, CodeGenerationError> {
        Ok(self.relative_path().join("mod").with_extension(Rust::extension()))
    }
}

#[cfg(test)]
mod tests {
    use crate::CodeGenerationConfig;
    use crate::generator::types::TypeGenerator;
    use crate::targets::rust::Rust;
    use reactive_graph_graph::Namespace;
    use reactive_graph_graph::Namespaces;
    use reactive_graph_graph::TypeResolver;

    #[test]
    pub fn test_generate_namespace_module() {
        let config = CodeGenerationConfig::with_temp_dir();
        let resolver = TypeResolver::new();
        let namespace = Namespace::random_path().unwrap();
        println!("fully qualified namespace: {namespace}");
        let target_path = TypeGenerator::<Rust>::absolute_target_path(&namespace, &config).unwrap();
        assert!(target_path.ends_with("mod.rs"));
        assert!(target_path.parent().unwrap().ends_with(namespace.last_segment().unwrap().to_string()));
        println!("target path: {target_path:?}");
        let code_gen_result = TypeGenerator::<Rust>::generate_type(&namespace, &config, &resolver);
        println!("{code_gen_result:?}");
        println!("resulting path: {:?}", code_gen_result.unwrap().path);
    }

    #[test]
    pub fn test_generate_namespace_modules() {
        let config = CodeGenerationConfig::with_temp_dir();
        let resolver = TypeResolver::new();
        let namespaces = Namespaces::random_type_tree().unwrap();
        for namespace in namespaces.iter() {
            println!("fully qualified namespace: {namespace}");
            let target_path = TypeGenerator::<Rust>::absolute_target_path(namespace, &config).unwrap();
            assert!(target_path.ends_with("mod.rs"));
            assert!(target_path.parent().unwrap().ends_with(namespace.last_segment().unwrap().to_string()));
            println!("target path: {target_path:?}");
            let code_gen_result = TypeGenerator::<Rust>::generate_type(namespace, &config, &resolver);
            println!("{code_gen_result:?}");
            println!("resulting path: {:?}", code_gen_result.unwrap().path);
        }
    }
}
