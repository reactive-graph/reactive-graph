#![feature(associated_type_bounds)]

#[macro_use]
extern crate darling;
extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

use quote::format_ident;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::Ident;

use crate::darling::FromDeriveInput;

#[derive(FromDeriveInput)]
#[darling(attributes(type_provider))]
struct TypeProviderConfig {
    tys: syn::Type,
    path: String,
    component_alias: Option<bool>,
}

uses_type_params!(TypeProviderConfig, tys);

#[proc_macro_derive(TypeProvider, attributes(type_provider))]
pub fn type_provider(input: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(input);

    let ident: Ident = input.ident.clone();
    let ident_assets = format_ident!("{}Assets", ident);

    let type_provider_config = match TypeProviderConfig::from_derive_input(&input) {
        Ok(type_provider_config) => type_provider_config,
        Err(e) => {
            return e.write_errors().into();
        }
    };
    let tys = type_provider_config.tys;
    let path = type_provider_config.path;
    let type_provider_id = ident.to_string();

    let component_alias = if type_provider_config.component_alias.unwrap_or(true) {
        quote! {
            #[reactive_graph_type_system_api::springtime_di::component_alias]
        }
    } else {
        TokenStream2::new()
    };

    #[cfg(feature = "json")]
    let json = {
        quote! {
            match reactive_graph_type_system_api::serde_json::from_str(asset_str) {
                Ok(parsed_entry) => {
                    let entry: <#tys as reactive_graph_graph::NamespacedTypeContainer>::Type = parsed_entry;
                    reactive_graph_graph::NamespacedTypeContainer::push(&entries, entry);
                }
                Err(e) => log::error!("Error in parsing JSON file {filename}: {e}"),
            }
        }
    };
    #[cfg(not(feature = "json"))]
    let json = {
        quote! {
            log::error!("Failed to read type definition from {filename}: JSON is not a supported file format!");
        }
    };

    #[cfg(feature = "json5")]
    let json5 = {
        quote! {
            match reactive_graph_type_system_api::json5::from_str(asset_str) {
                Ok(parsed_entry) => {
                    let entry: <#tys as reactive_graph_graph::NamespacedTypeContainer>::Type = parsed_entry;
                    reactive_graph_graph::NamespacedTypeContainer::push(&entries, entry);
                }
                Err(e) => log::error!("Error in parsing JSON5 file {filename}: {e}"),
            }
        }
    };
    #[cfg(not(feature = "json5"))]
    let json5 = {
        quote! {
            log::error!("Failed to read type definition from {filename}: JSON5 is not a supported file format!");
        }
    };

    #[cfg(feature = "toml")]
    let toml = {
        quote! {
            match reactive_graph_type_system_api::toml::from_str(asset_str) {
                Ok(parsed_entry) => {
                    let entry: <#tys as reactive_graph_graph::NamespacedTypeContainer>::Type = parsed_entry;
                    reactive_graph_graph::NamespacedTypeContainer::push(&entries, entry);
                }
                Err(e) => log::error!("Error in parsing TOML file {filename}: {e}"),
            }
        }
    };
    #[cfg(not(feature = "toml"))]
    let toml = {
        quote! {
            log::error!("Failed to read type definition from {filename}: TOML is not a supported file format!");
        }
    };

    let expanded = quote! {
        #[derive(rust_embed::RustEmbed)]
        #[folder = #path]
        struct #ident_assets;

        #[automatically_derived]
        #component_alias
        impl reactive_graph_type_system_api::TypeProvider<#tys> for #ident {
            fn id<'a>(&self) -> &'a str {
                #type_provider_id
            }
            fn get_types(&self) -> #tys {
                let mut entries = <#tys as reactive_graph_graph::NamespacedTypeContainer>::new();
                for file in #ident_assets::iter() {
                    let filename = file.as_ref();
                    if filename.starts_with(".") {
                        // do nothing
                        continue;
                    }
                    log::debug!("Loading resource {}", filename);
                    match #ident_assets::get(filename) {
                        Some(asset) => match std::str::from_utf8(asset.data.as_ref()) {
                            Ok(asset_str) => {
                                if filename.ends_with(".json") {
                                    #json
                                } else if filename.ends_with(".json5") {
                                    #json5
                                } else if filename.ends_with(".toml") {
                                    #toml
                                } else {
                                    log::error!("Can't read type definition {}: Only JSON, JSON5 and TOML are supported.", filename);
                                }
                            }
                            Err(e) => log::error!("Error in decoding file to UTF-8 {}: {}", filename, e),
                        },
                        None => {}
                    }
                }
                entries
            }
        }
    };
    TokenStream::from(expanded)
}
