use convert_case::Case;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use reactive_graph_graph::PropertyType;

#[inline]
pub fn const_property_variant_ident(property: &PropertyType) -> Ident {
    Ident::new(&property.name.to_case(Case::UpperSnake), Span::call_site())
}

#[inline]
pub fn property_name_ident(property: &PropertyType) -> Ident {
    Ident::new(&property.name, Span::call_site())
}
