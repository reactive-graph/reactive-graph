use crate::type_definition::rust::ident::AssocTypeDefinitionIdentType;
use convert_case::Casing;
use proc_macro2::Ident;
use proc_macro2::Span;
use quote::ToTokens;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use std::marker::PhantomData;
use std::ops::Deref;

pub struct TypeDefinitionIdent<IdentType: ?Sized>(TypeDefinition, Ident, PhantomData<IdentType>);

impl<IdentType: AssocTypeDefinitionIdentType + ?Sized> TypeDefinitionIdent<IdentType> {
    pub fn new<TY: TypeDefinitionGetter>(type_definition_getter: &TY) -> Self {
        let type_definition = type_definition_getter.type_definition();
        let name = IdentType::TYPE_DEFINITION_IDENT_TYPE.name(&type_definition);
        let postfix = IdentType::TYPE_DEFINITION_IDENT_TYPE.postfix();
        let ident = Ident::new(&format!("{name}{postfix}").to_case(IdentType::TYPE_DEFINITION_IDENT_TYPE.casing()), Span::call_site());
        Self(type_definition, ident, PhantomData)
    }
}

impl<T: TypeDefinitionGetter, IdentType: AssocTypeDefinitionIdentType + ?Sized> From<&T> for TypeDefinitionIdent<IdentType> {
    fn from(type_definition_getter: &T) -> Self {
        Self::new(type_definition_getter)
    }
}

impl<IdentType: AssocTypeDefinitionIdentType + ?Sized> Deref for TypeDefinitionIdent<IdentType> {
    type Target = Ident;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

impl<IdentType: AssocTypeDefinitionIdentType + ?Sized> AsRef<Ident> for TypeDefinitionIdent<IdentType> {
    fn as_ref(&self) -> &Ident {
        &self.1
    }
}

impl<IdentType: AssocTypeDefinitionIdentType + ?Sized> From<TypeDefinitionIdent<IdentType>> for Ident {
    fn from(type_definition_ident_types: TypeDefinitionIdent<IdentType>) -> Self {
        type_definition_ident_types.1
    }
}

impl<IdentType> From<TypeDefinitionIdent<IdentType>> for TypeDefinition {
    fn from(type_definition_ident_types: TypeDefinitionIdent<IdentType>) -> Self {
        type_definition_ident_types.0
    }
}

impl<IdentType> ToTokens for TypeDefinitionIdent<IdentType> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        self.1.to_tokens(tokens);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::type_definition::rust::ident::ConstNamespaceIdent;
    use reactive_graph_graph::ComponentTypeId;
    use std::str::FromStr;

    #[test]
    fn test_typed_type_definition_ident_constructor() {
        let ty = ComponentTypeId::from_str("test::Light").unwrap();
        let ident = TypeDefinitionIdent::<ConstNamespaceIdent>::new(&ty);
        assert_eq!("LIGHT_NAMESPACE", &ident.to_string());
    }

    #[test]
    fn test_typed_type_definition_ident_as_ref() {
        let ty = ComponentTypeId::from_str("test::Light").unwrap();
        let ident = TypeDefinitionIdent::<ConstNamespaceIdent>::from(&ty);
        assert_eq!("LIGHT_NAMESPACE", ident.as_ref().to_string());
    }

    #[test]
    fn test_typed_type_definition_ident_deref() {
        let ty = ComponentTypeId::from_str("test::Light").unwrap();
        let ident = TypeDefinitionIdent::<ConstNamespaceIdent>::from(&ty);
        assert_eq!("LIGHT_NAMESPACE", &ident.to_string());
    }
}
