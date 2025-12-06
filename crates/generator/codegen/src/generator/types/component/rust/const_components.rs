use crate::CodeGenerationError;
use crate::namespace::rust::fully_qualified_ident::FullyQualifiedNamespacedTypeIdent;
use crate::rust::Rust;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ConstComponentsIdent;
use crate::type_definition::rust::ConstTypeIdIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::DocumentationConfig;
use reactive_graph_generator_documentation::DocumentationConfigPreset;
use reactive_graph_generator_documentation::FromDocumentationConfigPreset;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::NamespacedTypeIdContainer;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;

pub fn generate_const_components<TY: TypeDefinitionGetter + ComponentTypeIdContainer>(
    type_: &TY,
    resolver: &TypeResolver,
) -> Result<TokenStream, CodeGenerationError> {
    let const_components_ident = ConstComponentsIdent::new(type_);

    let component_tys = type_.get_components_cloned();
    let doc_comment = Rust::multiline_doc_comment(const_components_doc_comment(&component_tys, resolver));

    let mut components = component_tys.to_vec();
    components.sort();
    let mut component_idents = Vec::new();
    for ty in components.iter() {
        component_idents.push(ty.fully_qualified_ident_of_type::<ConstTypeIdIdent>(resolver)?);
    }

    Ok(quote! {
        #[doc(newline)]
        #doc_comment
        pub static #const_components_ident: std::sync::LazyLock<reactive_graph_graph::ComponentTypeIds> = std::sync::LazyLock::new(|| {
            reactive_graph_graph::ComponentTypeIds::new()
                #(.component(std::ops::Deref::deref(&#component_idents)))*
        });
    })
}

#[inline]
fn const_components_doc_comment<TY: ComponentTypeIdContainer + GenerateDocumentation<TY>>(type_: &TY, resolver: &TypeResolver) -> String {
    type_
        .generate_documentation(&DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short), resolver)
        .map(|documentation| format!(" {}", documentation.to_string()))
        .unwrap_or_default()
}
