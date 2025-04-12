extern crate proc_macro;

use proc_macro::TokenStream;

use quote::ToTokens;
use quote::TokenStreamExt;
use quote::format_ident;
use quote::quote;
use syn::DeriveInput;
use syn::Ident;
use syn::PathArguments;
use syn::Type;
use syn::parse_macro_input;

use darling::Error;
use darling::FromDeriveInput;
use darling::FromMeta;
use darling::ast::NestedMeta;
use proc_macro2::Span;

#[derive(FromMeta)]
struct ReactiveEntityConfig {
    pub namespace: String,
    pub type_name: String,
}

#[proc_macro_attribute]
pub fn reactive_entity(args: TokenStream, item: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(item);
    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(attr_args) => attr_args,
        Err(e) => {
            return TokenStream::from(Error::from(e).write_errors());
        }
    };
    let reactive_entity_config = match ReactiveEntityConfig::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return TokenStream::from(e.write_errors());
        }
    };

    let ident = input.ident.clone();
    let namespace = reactive_entity_config.namespace;
    let type_name = reactive_entity_config.type_name;

    let output = match input.clone().data {
        syn::Data::Struct(s) => match s.fields {
            syn::Fields::Named(mut fields) => fields
                .named
                .iter_mut()
                .map(|field| {
                    let field_name = &field.ident.clone().unwrap();
                    let field_type = &field.ty.clone();
                    let field_vis = &field.vis;
                    quote! {
                        #field_vis #field_name: reactive_graph_reactive_service_api::TypedReactivePropertyImpl<uuid::Uuid, reactive_graph_reactive_model_impl::ReactiveEntity, #field_type>,
                    }
                })
                .collect(),
            _ => quote!(),
        },
        _ => quote!(),
    };

    let expanded = quote! {
        #[derive(reactive_graph_reactive_service_api::ReactiveEntity)]
        #[reactive_entity_derive(namespace = #namespace, type_name = #type_name)]
        pub struct #ident {
            // reactive_instance: reactive_graph_reactive_service_api::ReactiveEntity,
            #output
        }
    }
    .to_token_stream();
    TokenStream::from(expanded)
}

#[derive(FromDeriveInput)]
#[darling(attributes(reactive_entity_derive))]
struct ReactiveEntityDeriveConfig {
    pub namespace: String,
    pub type_name: String,
}

#[proc_macro_derive(ReactiveEntity, attributes(reactive_entity_derive))]
pub fn reactive_entity_derive(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let ident: Ident = input.ident.clone();

    let config = match ReactiveEntityDeriveConfig::from_derive_input(&input) {
        Ok(config) => config,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    let ident_entity_type = format_ident!("ENTITY_TYPE_ID__{}__{}", config.namespace.to_uppercase(), config.type_name.to_uppercase());
    let namespace = config.namespace;
    let type_name = config.type_name;

    let mut constructor_fields = quote! {};
    let mut constructor_parameter_bounds = quote! {};
    let mut constructor_parameters = quote! {};
    let mut constructor_properties = quote! {};
    if let syn::Data::Struct(s) = input.data {
        if let syn::Fields::Named(fields) = s.fields {
            constructor_fields.append_all(fields.named.iter().map(|f| {
                let field_name = f.ident.clone().unwrap();
                let property_name = field_name.to_string();
                quote! {
                    #field_name: reactive_graph_reactive_service_api::TypedReactivePropertyConstructor::new(reactive_instance.clone(), #property_name),
                }
            }));

            constructor_parameter_bounds.append_all(fields.named.iter().map(|f| {
                let parameter_bound = format_ident!("{}", f.ident.clone().unwrap().to_string().to_uppercase());
                let field_ty = match f.ty.clone() {
                    Type::Path(p) => {
                        let target_types: Vec<_> = p
                            .path
                            .segments
                            .iter()
                            .filter(|seg| seg.ident == Ident::new("TypedReactivePropertyImpl", Span::call_site()))
                            .cloned()
                            .map(|seg| seg.arguments)
                            .filter_map(|args| match args {
                                PathArguments::AngleBracketed(args) => Some(args),
                                _ => None,
                            })
                            .map(|type_parameters| type_parameters.args)
                            .filter_map(|type_parameters| type_parameters.last().cloned())
                            .collect();
                        let target_type = target_types.first().cloned().unwrap();
                        target_type.to_token_stream()
                    }
                    _ => {
                        quote! {}
                    }
                };
                quote! {
                    #parameter_bound: Into<#field_ty>,
                }
            }));

            constructor_parameters.append_all(fields.named.iter().map(|f| {
                let field_name = f.ident.clone().unwrap();
                let parameter_bound = format_ident!("{}", field_name.to_string().to_uppercase());
                quote! {
                    #field_name: #parameter_bound,
                }
            }));

            constructor_properties.append_all(fields.named.iter().map(|f| {
                let field_name = f.ident.clone().unwrap();
                let property_name = field_name.to_string();
                quote! {
                    properties.insert(#property_name.to_string(), serde_json::json!(#field_name.into()));
                    // properties.insert(#property_name.to_string(), serde_json::json!(reactive_graph_reactive_service_api::TypedReactivePropertyAccessor::get(&#field_name.into())));
                }
            }));
        }
    }

    let expanded = quote! {

        pub static #ident_entity_type: std::sync::LazyLock<reactive_graph_graph::EntityTypeId> = std::sync::LazyLock::new(|| { reactive_graph_graph::EntityTypeId::new_from_type(#namespace, #type_name)});

        #[automatically_derived]
        impl From<reactive_graph_reactive_model_impl::ReactiveEntity> for #ident {
            fn from(reactive_instance: reactive_graph_reactive_model_impl::ReactiveEntity) -> Self {
                Self {
                    // reactive_instance: reactive_instance.clone(),
                    #constructor_fields
                }
            }
        }
        #[automatically_derived]
        impl From<&reactive_graph_reactive_model_impl::ReactiveEntity> for #ident {
            fn from(reactive_instance: &reactive_graph_reactive_model_impl::ReactiveEntity) -> Self {
                Self {
                    // reactive_instance: reactive_instance.clone(),
                    #constructor_fields
                }
            }
        }

        #[automatically_derived]
        impl #ident {
            pub fn new
                <#constructor_parameter_bounds>
            (
                #constructor_parameters
            ) -> Self {
                let id = uuid::Uuid::new_v4();
                let mut properties = reactive_graph_graph::PropertyInstances::new();
                #constructor_properties
                // properties.insert("value".to_string(), serde_json::json!(value.into()));
                let properties = reactive_graph_reactive_model_impl::ReactiveProperties::new_with_id_from_properties(id, properties);
                let ty = std::ops::Deref::deref(&#ident_entity_type).clone();
                Self::from(reactive_graph_reactive_model_impl::ReactiveEntity::builder().ty(ty).id(id).properties(properties).build())
            }
        }

        #[automatically_derived]
        impl reactive_graph_reactive_service_api::TypedReactivePropertyContainer<reactive_graph_graph::EntityTypeId, reactive_graph_graph::EntityType> for #ident {
            fn new_with_ty<TY: Into<reactive_graph_graph::EntityTypeId>>(ty: TY) -> Self {
                #ident::from(reactive_graph_reactive_model_impl::ReactiveEntity::builder().ty(ty).id(uuid::Uuid::new_v4()).build())
            }

            fn new_from_type(entity_type: &reactive_graph_graph::EntityType) -> Self {
                #ident::from(reactive_graph_reactive_model_impl::ReactiveEntity::builder_from_entity_type(&entity_type).build())
            }
        }

        #[automatically_derived]
        impl Default for #ident {
            fn default() -> Self {
                let ty = std::ops::Deref::deref(&#ident_entity_type).clone();
                #ident::from(reactive_graph_reactive_model_impl::ReactiveEntity::builder().ty(ty).id(uuid::Uuid::new_v4()).build())
            }
        }

        // #[automatically_derived]
        // impl reactive_graph_reactive_service_api::ReactiveInstanceGetter<reactive_graph_reactive_model_impl::ReactiveEntity> for #ident {
        //     fn get_reactive_instance() -> &reactive_graph_reactive_model_impl::ReactiveEntity {
        //         &self.reactive_instance
        //     }
        // }
    };
    TokenStream::from(expanded)
}

// #[proc_macro_derive(Trigger)]
// pub fn reactiveInstanceTrigger(input: TokenStream) -> TokenStream {
//     let input: DeriveInput = parse_macro_input!(input);
//     let expanded = quote! {
//         impl Action for #ident {
//             fn trigger(&self) {
//                 self.reactive_instance
//             }
//         }
//     }
//     TokenStream::from(expanded)
// }
