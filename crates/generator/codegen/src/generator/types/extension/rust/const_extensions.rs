use crate::rust::Rust;
use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::ConstExtensionsIdent;
use proc_macro2::TokenStream;
use quote::quote;
use reactive_graph_generator_documentation::DocumentationConfig;
use reactive_graph_generator_documentation::DocumentationConfigPreset;
use reactive_graph_generator_documentation::FromDocumentationConfigPreset;
use reactive_graph_generator_documentation::GenerateDocumentation;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeResolver;

pub fn generate_const_extensions<TY: TypeDefinitionGetter + ExtensionContainer>(type_: &TY, resolver: &TypeResolver) -> TokenStream {
    let const_extensions_ident = ConstExtensionsIdent::new(type_);

    let extensions = type_.get_own_extensions_cloned();
    let doc_comment = Rust::multiline_doc_comment(const_extensions_doc_comment(&extensions, resolver));

    let mut extensions = extensions.to_vec();
    extensions.sort();
    let mut extensions_token_stream = Vec::new();
    for extension in extensions.iter() {
        let extension_ty = extension.ty.namespace().to_string();
        let extension_ty = quote! {
            .ty(reactive_graph_graph::ExtensionTypeId::parse_str(#extension_ty).unwrap())
        };
        let entity_ty = match &extension.entity_ty {
            None => quote! {},
            Some(entity_ty) => {
                let entity_ty = entity_ty.namespace().to_string();
                quote! {
                    .entity_ty(reactive_graph_graph::EntityTypeId::parse_str(#entity_ty).unwrap())
                }
            }
        };
        let description = extension.description.clone();
        let extension = match serde_json::to_string_pretty(&extension.extension) {
            Ok(extension_serialized) => {
                // let multiline_literal = LitStr::new(&extension_serialized, Span::call_site());
                let raw_string_literal = format!("r#\"{}\"#", extension_serialized.trim());
                let token_stream: TokenStream = raw_string_literal.parse().unwrap();
                quote! {
                    serde_json::from_str(#token_stream).unwrap()
                }
            }
            Err(_) => quote! {},
        };
        extensions_token_stream.push(quote! {
            reactive_graph_graph::Extension::builder()
                #extension_ty
                #entity_ty
                .description(#description)
                .extension(#extension)
                .build()
        });
    }
    quote! {
        #[doc(newline)]
        #doc_comment
        pub static #const_extensions_ident: std::sync::LazyLock<reactive_graph_graph::Extensions> = std::sync::LazyLock::new(|| {
            reactive_graph_graph::Extensions::new()
            #(.extension(#extensions_token_stream))*
        });
    }
}

#[inline]
pub fn const_extensions_doc_comment<TY: ExtensionContainer + GenerateDocumentation<TY>>(type_: &TY, resolver: &TypeResolver) -> String {
    type_
        .generate_documentation(&DocumentationConfig::new_from_preset(DocumentationConfigPreset::Short), resolver)
        .map(|documentation| format!(" {}", documentation.to_string()))
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::extension::rust::generate_const_extensions;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::ExtensionTypeId;
    use reactive_graph_graph::Extensions;
    use reactive_graph_graph::TypeResolver;
    use serde_json::json;

    #[test]
    fn test_extensions() {
        let component = ComponentTypeId::parse_str("test::Component")
            .unwrap()
            .into_builder()
            .extensions(
                Extensions::new().extension(
                    ExtensionTypeId::parse_str("text::Extension")
                        .unwrap()
                        .into_builder()
                        .extension(json! {
                            {
                                "test": "test",
                            }
                        })
                        .build(),
                ),
            )
            .build();

        let resolver = TypeResolver::new();
        let token_stream = generate_const_extensions(&component, &resolver);
        let content = token_stream.to_string();
        println!("{}", token_stream.to_string());
        assert!(content.contains("serde_json :: from_str"));
        assert!(content.contains("(r#\"{\n"));
        assert!(content.contains("\"test\": \"test\"\n"));
        assert!(content.contains("}\"#)"));
    }
}
