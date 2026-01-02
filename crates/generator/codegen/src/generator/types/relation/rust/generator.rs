use crate::CodeFormatter;
use crate::CodeGenerationConfig;
use crate::CodeGenerationError;
use crate::CodeGenerationResult;
use crate::CodeGenerationTarget;
use crate::TypeGenerator;
use crate::component::rust::generate_const_components;
use crate::extension::rust::generate_const_extensions;
use crate::header::rust::generate_header_generated_code;
use crate::namespace::rust::generate_const_namespace;
use crate::property::rust::generate_const_properties;
use crate::relation::rust::const_type_definition::generate_const_type_definition;
use crate::relation::rust::struct_definition::generate_struct_definition;
use crate::rust::Rust;
use crate::type_definition::rust::generate_const_type_id;
use convert_case::Case::Snake;
use convert_case::Casing;
use quote::quote;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationType;
use reactive_graph_graph::TypeResolver;
use std::path::PathBuf;

impl TypeGenerator<Rust> for RelationType {
    fn generate_type(&self, config: &CodeGenerationConfig, resolver: &TypeResolver) -> Result<CodeGenerationResult, CodeGenerationError> {
        let path = TypeGenerator::<Rust>::absolute_target_path(self, config)?;
        let namespace = self.namespace();
        let Some(parent_namespace) = namespace.parent() else {
            return Err(CodeGenerationError::ParentNamespaceError(namespace));
        };
        parent_namespace.generate_type(config, resolver)?;
        parent_namespace.register_child(config, resolver, &namespace)?;

        println!("Generating source code for relation type {} to {path:?}", self.namespace());

        let properties = resolver.resolve_properties_sorted(self)?;

        let header_generated_code = generate_header_generated_code();
        let const_namespace = generate_const_namespace(self);
        let const_type_id = generate_const_type_id(self);
        let const_properties = generate_const_properties(self, resolver, &properties)?;
        let const_components = generate_const_components(self, resolver)?;
        let const_extensions = generate_const_extensions(self, resolver);
        let const_type_definition = generate_const_type_definition(self, resolver)?;
        let struct_definition = generate_struct_definition(self, config, resolver, &properties);

        let token_stream = quote! {
            #header_generated_code
            #const_namespace
            #const_type_id
            #const_properties
            #const_components
            #const_extensions
            #const_type_definition
            #struct_definition
            // #impl_trait_property_instance_getter_and_setter
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
