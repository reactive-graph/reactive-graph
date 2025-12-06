pub use super::types::*;
use reactive_graph_graph::TypeDefinitionGetter;

pub trait AssocTypeDefinitionIdentType {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes;

    fn new<TY: TypeDefinitionGetter>(type_definition_getter: &TY) -> super::TypeDefinitionIdent<Self> {
        super::TypeDefinitionIdent::<Self>::new(type_definition_getter)
    }
}

pub enum TypeTypeIdIdent {}

impl AssocTypeDefinitionIdentType for TypeTypeIdIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::TypeTypeId;
}

pub enum TypeTypeIdent {}

impl AssocTypeDefinitionIdentType for TypeTypeIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::TypeType;
}

pub enum ConstNamespaceIdent {}

impl AssocTypeDefinitionIdentType for ConstNamespaceIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::ConstNamespace;
}

pub enum ConstTypeIdIdent {}

impl AssocTypeDefinitionIdentType for ConstTypeIdIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::ConstTypeId;
}

pub enum TypePropertiesIdent {}

impl AssocTypeDefinitionIdentType for TypePropertiesIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::TypeProperties;
}

pub enum TypePropertiesIteratorIdent {}

impl AssocTypeDefinitionIdentType for TypePropertiesIteratorIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::TypePropertiesIterator;
}

pub enum ConstComponentsIdent {}

impl AssocTypeDefinitionIdentType for ConstComponentsIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::ConstComponents;
}

pub enum ConstExtensionsIdent {}

impl AssocTypeDefinitionIdentType for ConstExtensionsIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::ConstExtensions;
}

pub enum ConstTypeIdent {}

impl AssocTypeDefinitionIdentType for ConstTypeIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::ConstType;
}

pub enum TypeIdent {}

impl AssocTypeDefinitionIdentType for TypeIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::Type;
}

pub enum TypeValueIdent {}

impl AssocTypeDefinitionIdentType for TypeValueIdent {
    const TYPE_DEFINITION_IDENT_TYPE: TypeDefinitionIdentTypes = TypeDefinitionIdentTypes::TypeValue;
}

#[cfg(test)]
mod tests {
    use super::*;
    use proc_macro2::Ident;
    use proc_macro2::Span;
    use quote::quote;
    use reactive_graph_graph::ComponentTypeId;
    use reactive_graph_graph::EntityTypeId;
    use std::ops::Deref;
    use std::str::FromStr;

    #[test]
    fn test_type_type_id_ident() {
        let ident = TypeTypeIdIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("ComponentTypeId", &ident.to_string());
        let ident = TypeTypeIdIdent::new(&EntityTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("EntityTypeId", &ident.to_string());
    }

    #[test]
    fn test_type_type_ident() {
        let ident = TypeTypeIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("Component", &ident.to_string());
        let ident = TypeTypeIdent::new(&EntityTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("EntityType", &ident.to_string());
    }

    #[test]
    fn test_const_namespace_ident() {
        let ty = ComponentTypeId::from_str("test::LightSource").unwrap();
        let ident = ConstNamespaceIdent::new(&ty);
        assert_eq!(&Ident::new("LIGHT_SOURCE_NAMESPACE", Span::call_site()), ident.deref());
    }

    #[test]
    fn test_const_namespace_ident_token_stream() {
        let ty = ComponentTypeId::from_str("test::LightSource").unwrap();
        let ident = ConstNamespaceIdent::new(&ty);
        let token_stream = quote! {#ident};
        let expected_ident = Ident::new("LIGHT_SOURCE_NAMESPACE", Span::call_site());
        let expected_token_stream = quote! {#expected_ident};
        assert_eq!(token_stream.to_string(), expected_token_stream.to_string());
    }

    #[test]
    fn test_const_type_id_ident() {
        let ident = ConstTypeIdIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LIGHT_SOURCE", &ident.to_string());
    }

    #[test]
    fn test_type_properties_ident() {
        let ident = TypePropertiesIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LightSourceProperties", &ident.to_string());
    }

    #[test]
    fn test_type_properties_iterator_ident() {
        let ident = TypePropertiesIteratorIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LightSourcePropertiesIterator", &ident.to_string());
    }

    #[test]
    fn test_const_components_ident() {
        let ident = ConstComponentsIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LIGHT_SOURCE_COMPONENTS", &ident.to_string());
    }

    #[test]
    fn test_const_extensions_ident() {
        let ident = ConstExtensionsIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LIGHT_SOURCE_EXTENSIONS", &ident.to_string());
    }

    #[test]
    fn test_const_type_ident() {
        let ident = ConstTypeIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LIGHT_SOURCE_TYPE", &ident.to_string());
    }

    #[test]
    fn test_type_ident() {
        let ident = TypeIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("LightSource", &ident.to_string());
    }

    #[test]
    fn test_type_value_ident() {
        let ident = TypeValueIdent::new(&ComponentTypeId::from_str("test::LightSource").unwrap());
        assert_eq!("light_source", &ident.to_string());
    }
}
