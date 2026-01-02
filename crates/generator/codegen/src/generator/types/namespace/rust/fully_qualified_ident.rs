use crate::CodeGenerationError;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypeDefinitionIdent;
use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;
use std::ops::Deref;

pub trait NamespacedTypeIdent {
    fn ident_of_type<IdentType: AssocTypeDefinitionIdentType + ?Sized>(&self) -> Result<Ident, CodeGenerationError>;
}

impl<TY: NamespacedTypeGetter + TypeDefinitionGetter> NamespacedTypeIdent for TY {
    fn ident_of_type<IdentType: AssocTypeDefinitionIdentType + ?Sized>(&self) -> Result<Ident, CodeGenerationError> {
        Ok(TypeDefinitionIdent::<IdentType>::new(self).deref().clone())
    }
}

pub trait FullyQualifiedNamespacedTypeIdent: NamespacedTypeIdent {
    fn crate_name(&self, resolver: &TypeResolver) -> Result<Ident, CodeGenerationError>;
    fn fully_qualified_module(&self) -> Result<TokenStream, CodeGenerationError>;
    fn fully_qualified_ident_of_type<IdentType: AssocTypeDefinitionIdentType + ?Sized>(
        &self,
        resolver: &TypeResolver,
    ) -> Result<TokenStream, CodeGenerationError>;
}

impl<TY: NamespacedTypeGetter + TypeDefinitionGetter> FullyQualifiedNamespacedTypeIdent for TY {
    fn crate_name(&self, resolver: &TypeResolver) -> Result<Ident, CodeGenerationError> {
        let crate_name = resolver.crate_name(self).ok_or(CodeGenerationError::NamespaceParseError(self.namespace()))?;
        if crate_name.is_empty() || crate_name == "build_script_build" {
            Ok(Ident::new("crate", Span::call_site()))
        } else {
            Ok(Ident::new(&crate_name.to_case(Case::Snake), Span::call_site()))
        }
    }

    fn fully_qualified_module(&self) -> Result<TokenStream, CodeGenerationError> {
        let path = self.path();
        let fully_qualified_module = format!("{}::{}", path.to_string(), self.type_name().to_case(Case::Snake));
        str::parse::<TokenStream>(&fully_qualified_module).map_err(|_| CodeGenerationError::NamespaceParseError(path))
    }

    fn fully_qualified_ident_of_type<IdentType: AssocTypeDefinitionIdentType + ?Sized>(
        &self,
        resolver: &TypeResolver,
    ) -> Result<TokenStream, CodeGenerationError> {
        let crate_ident = self.crate_name(resolver)?;
        let fully_qualified_module = self.fully_qualified_module()?;
        let ident = self.ident_of_type::<IdentType>()?;
        Ok(quote! {
            #crate_ident::#fully_qualified_module::#ident
        })
    }
}
