use proc_macro2::TokenStream;
use quote::ToTokens;
use reactive_graph_graph::DataType;
use reactive_graph_graph::PropertyType;
use syn::Path;
use syn::parse_quote;

pub fn data_type_token_stream(property: &PropertyType) -> TokenStream {
    let property_data_type: Path = match property.data_type {
        DataType::Null => parse_quote!(reactive_graph_graph::DataType::Null),
        DataType::Bool => parse_quote!(reactive_graph_graph::DataType::Bool),
        DataType::Number => parse_quote!(reactive_graph_graph::DataType::Number),
        DataType::String => parse_quote!(reactive_graph_graph::DataType::String),
        DataType::Array => parse_quote!(reactive_graph_graph::DataType::Array),
        DataType::Object => parse_quote!(reactive_graph_graph::DataType::Object),
        DataType::Any => parse_quote!(reactive_graph_graph::DataType::Any),
    };
    property_data_type.to_token_stream()
}
