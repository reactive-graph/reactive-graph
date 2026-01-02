use proc_macro2::TokenStream;
use quote::ToTokens;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::SocketType;
use syn::Path;
use syn::parse_quote;

pub fn socket_type_token_stream(property: &PropertyType) -> TokenStream {
    let property_socket_type: Path = match property.socket_type {
        SocketType::None => parse_quote!(reactive_graph_graph::SocketType::None),
        SocketType::Input => parse_quote!(reactive_graph_graph::SocketType::Input),
        SocketType::Output => parse_quote!(reactive_graph_graph::SocketType::Output),
    };
    property_socket_type.to_token_stream()
}
