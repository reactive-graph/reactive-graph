use proc_macro2::TokenStream;
use quote::ToTokens;
use reactive_graph_graph::Mutability;
use reactive_graph_graph::PropertyType;
use syn::Path;
use syn::parse_quote;

pub fn mutability_token_stream(property: &PropertyType) -> TokenStream {
    let property_mutability: Path = match property.mutability {
        Mutability::Mutable => parse_quote!(reactive_graph_graph::Mutability::Mutable),
        Mutability::Immutable => parse_quote!(reactive_graph_graph::Mutability::Immutable),
    };
    property_mutability.to_token_stream()
}
