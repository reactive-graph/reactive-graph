use crate::rust::Rust;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ConstNamespaceIdent;
use crate::type_definition::rust::ConstTypeIdIdent;
use crate::type_definition::rust::TypeTypeIdIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;

pub fn generate_const_type_id<TY: TypeDefinitionGetter>(type_: &TY) -> TokenStream {
    let type_definition = type_.type_definition();
    let const_namespace_ident = ConstNamespaceIdent::new(type_);
    let type_id_type_ident = TypeTypeIdIdent::new(type_);
    let const_type_id_ident = ConstTypeIdIdent::new(type_);
    let doc_comment = Rust::multiline_doc_comment(const_type_id_doc_comment(&type_definition));

    quote! {
        #[doc(newline)]
        #doc_comment
        pub static #const_type_id_ident: std::sync::LazyLock<reactive_graph_graph::#type_id_type_ident> = std::sync::LazyLock::new(
            || std::str::FromStr::from_str(#const_namespace_ident).unwrap()
        );
    }
}

#[inline]
fn const_type_id_doc_comment(type_definition: &TypeDefinition) -> String {
    format!(
        "The [type identifier]() of {} `{}`.\n\n### Namespace\n\nThe fully qualified namespace is\n `{}`",
        type_definition.type_id_type.full_name(),
        type_definition.type_name().to_string(),
        type_definition.namespace().to_string(),
    )
}
