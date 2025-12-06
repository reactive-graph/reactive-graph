use crate::CodeFormatter;
use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::CodeGenerationResult;
use crate::CodeGenerationTarget;
use crate::TypeGenerator;
use crate::namespace::rust::fully_qualified_ident::FullyQualifiedNamespacedTypeIdent;
use crate::namespace::rust::sort_file_items;
use crate::rust::Rust;
use crate::type_definition::rust::ConstTypeIdent;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::TypeResolver;
use reactive_graph_graph::TypeSystem;
use std::fs::File;
use std::fs::create_dir_all;
use std::path::PathBuf;
use syn::parse_quote;

impl TypeGenerator<Rust> for TypeSystem {
    fn generate_type(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        self.register_root(config, resolver)?;

        let mut component_idents = Vec::new();
        for ty in self.components().iter() {
            component_idents.push(ty.fully_qualified_ident_of_type::<ConstTypeIdent>(resolver)?);
        }
        let mut entity_type_idents = Vec::new();
        for ty in self.entity_types().iter() {
            entity_type_idents.push(ty.fully_qualified_ident_of_type::<ConstTypeIdent>(resolver)?);
        }
        let mut relation_type_idents = Vec::new();
        for ty in self.relation_types().iter() {
            relation_type_idents.push(ty.fully_qualified_ident_of_type::<ConstTypeIdent>(resolver)?);
        }
        // TODO: Implement
        let mut flow_type_idents: Vec<TokenStream> = Vec::new();
        // for ty in self.flow_types().iter() {
        //     flow_type_idents.push(ty.fully_qualified_ident_of_type::<ConstTypeIdIdent>(resolver)?);
        // }
        // #(.component(std::ops::Deref::deref(&#component_idents)))*
        // #(.entity(std::ops::Deref::deref(&#entity_type_idents)))*
        // #(.relation(std::ops::Deref::deref(&#relation_type_idents)))*
        // #(.flow(std::ops::Deref::deref(&#flow_type_idents)))*
        // .entity_types(&TYPE_SYSTEM_ENTITY_TYPES)
        //                     .relation_types(&TYPE_SYSTEM_RELATION_TYPES)
        //                     .flow_types(&TYPE_SYSTEM_FLOW_TYPES)

        let type_system_id = match &config.id {
            Some(id) => {
                let id = id.to_string();
                quote! {
                    pub static TYPE_SYSTEM_ID: &str = #id;
                    pub static TYPE_SYSTEM_NAMESPACE: std::sync::LazyLock<reactive_graph_graph::Namespace> = std::sync::LazyLock::new(|| {
                        std::str::FromStr::from_str(TYPE_SYSTEM_ID).unwrap()
                    });
                }
            }
            None => quote! {},
        };
        let token_stream = quote! {
            #[doc(newline)]
            pub static TYPE_SYSTEM_COMPONENTS: std::sync::LazyLock<reactive_graph_graph::Components> = std::sync::LazyLock::new(|| {
                reactive_graph_graph::Components::new()
                    #(.component(#component_idents.clone()))*
            });

            #[doc(newline)]
            pub static TYPE_SYSTEM_ENTITY_TYPES: std::sync::LazyLock<reactive_graph_graph::EntityTypes> = std::sync::LazyLock::new(|| {
                reactive_graph_graph::EntityTypes::new()
                    #(.entity(#entity_type_idents.clone()))*
            });

            #[doc(newline)]
            pub static TYPE_SYSTEM_RELATION_TYPES: std::sync::LazyLock<reactive_graph_graph::RelationTypes> = std::sync::LazyLock::new(|| {
                reactive_graph_graph::RelationTypes::new()
                    #(.relation(#relation_type_idents.clone()))*
            });

            #[doc(newline)]
            pub static TYPE_SYSTEM_FLOW_TYPES: std::sync::LazyLock<reactive_graph_graph::FlowTypes> = std::sync::LazyLock::new(|| {
                reactive_graph_graph::FlowTypes::new()
                    #(.flow(#flow_type_idents.clone()))*
            });

            #[doc(newline)]
            pub static TYPE_SYSTEM: std::sync::LazyLock<reactive_graph_graph::TypeSystem> = std::sync::LazyLock::new(|| {
                reactive_graph_graph::TypeSystem::builder()
                    .components(TYPE_SYSTEM_COMPONENTS.clone())
                    .entity_types(TYPE_SYSTEM_ENTITY_TYPES.clone())
                    .relation_types(TYPE_SYSTEM_RELATION_TYPES.clone())
                    .flow_types(TYPE_SYSTEM_FLOW_TYPES.clone())
                    .build()
            });

            #type_system_id

        };

        let output = Rust::format(token_stream.to_string(), config).map_err(|e| CodeGenerationError::FormatterError(path.clone(), e))?;
        std::fs::write(&path, output).map_err(|e| CodeGenerationError::WriteError(path.clone(), e))?;
        Ok(CodeGenerationResult { path })
    }

    fn register_root(&self, config: &CodeGenerationConfig, _resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let root_module_name = "type_system";
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
        Ok(PathBuf::from("type_system.rs"))
    }
}
