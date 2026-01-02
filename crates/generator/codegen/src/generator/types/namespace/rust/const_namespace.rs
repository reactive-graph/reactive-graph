use crate::rust::Rust;
use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ident::ConstNamespaceIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_const_namespace<TY: TypeDefinitionGetter>(type_: &TY) -> TokenStream {
    let type_definition = type_.type_definition();
    let fully_qualified_namespace = type_definition.namespace().to_string();
    let namespace_const_ident = ConstNamespaceIdent::new(type_);
    let doc_comment = Rust::multiline_doc_comment(format!("Namespace `{fully_qualified_namespace}`"));
    quote! {
        #[doc(newline)]
        #doc_comment
        #[cfg_attr(rustfmt, rustfmt_skip)]
        pub const #namespace_const_ident: &str = #fully_qualified_namespace;
    }
}
