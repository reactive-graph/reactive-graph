use crate::type_definition::rust::AssocTypeDefinitionIdentType;
use crate::type_definition::rust::TypeDefinitionIdent;
use convert_case::Case;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeIdType;

pub enum TypeDefinitionIdentTypes {
    TypeTypeId, // TODO: TypeTypeId (Component"TypeId")
    TypeType,   // TODO: TypeType (Component"Type")
    ConstNamespace,
    ConstTypeId,
    TypeProperties,
    TypePropertiesIterator,
    ConstComponents,
    ConstExtensions,
    ConstType,
    Type,
    TypeValue,
}

impl TypeDefinitionIdentTypes {
    pub fn into_ident<TY: TypeDefinitionGetter, IdentType: AssocTypeDefinitionIdentType + ?Sized>(self, ty: &TY) -> TypeDefinitionIdent<IdentType> {
        TypeDefinitionIdent::<IdentType>::new(ty)
    }

    pub fn casing(&self) -> Case<'_> {
        match self {
            TypeDefinitionIdentTypes::TypeTypeId => Case::UpperCamel,
            TypeDefinitionIdentTypes::TypeType => Case::UpperCamel,
            TypeDefinitionIdentTypes::ConstNamespace => Case::UpperSnake,
            TypeDefinitionIdentTypes::ConstTypeId => Case::UpperSnake,
            TypeDefinitionIdentTypes::TypeProperties => Case::UpperCamel,
            TypeDefinitionIdentTypes::TypePropertiesIterator => Case::UpperCamel,
            TypeDefinitionIdentTypes::ConstComponents => Case::UpperSnake,
            TypeDefinitionIdentTypes::ConstExtensions => Case::UpperSnake,
            TypeDefinitionIdentTypes::ConstType => Case::UpperSnake,
            TypeDefinitionIdentTypes::Type => Case::UpperCamel,
            TypeDefinitionIdentTypes::TypeValue => Case::Snake,
        }
    }

    pub fn postfix(&self) -> String {
        match self {
            TypeDefinitionIdentTypes::TypeTypeId => "TypeId".to_string(),
            TypeDefinitionIdentTypes::TypeType => String::new(), // TODO: "Type".to_string(),
            TypeDefinitionIdentTypes::ConstNamespace => "Namespace".to_string(),
            TypeDefinitionIdentTypes::ConstTypeId => String::new(),
            TypeDefinitionIdentTypes::TypeProperties => "Properties".to_string(),
            TypeDefinitionIdentTypes::TypePropertiesIterator => "PropertiesIterator".to_string(),
            TypeDefinitionIdentTypes::ConstComponents => "Components".to_string(),
            TypeDefinitionIdentTypes::ConstExtensions => "Extensions".to_string(),
            TypeDefinitionIdentTypes::ConstType => "Type".to_string(),
            TypeDefinitionIdentTypes::Type => String::new(),
            TypeDefinitionIdentTypes::TypeValue => String::new(),
        }
    }

    pub fn name(&self, type_definition: &TypeDefinition) -> String {
        match self {
            TypeDefinitionIdentTypes::TypeTypeId => match type_definition.type_id_type {
                TypeIdType::Behaviour => "Behaviour",
                TypeIdType::Component => "Component",
                TypeIdType::EntityType => "Entity",
                TypeIdType::Extension => "Extension",
                TypeIdType::RelationType => "Relation",
                TypeIdType::FlowType => "Flow",
            }
            .to_string(),
            TypeDefinitionIdentTypes::TypeType => match type_definition.type_id_type {
                TypeIdType::Behaviour => "BehaviourType",
                TypeIdType::Component => "Component", // fix this to make postfixes safe to use!
                TypeIdType::EntityType => "EntityType",
                TypeIdType::Extension => "ExtensionType",
                TypeIdType::RelationType => "RelationType",
                TypeIdType::FlowType => "FlowType",
            }
            .to_string(),
            TypeDefinitionIdentTypes::ConstNamespace
            | TypeDefinitionIdentTypes::ConstTypeId
            | TypeDefinitionIdentTypes::TypeProperties
            | TypeDefinitionIdentTypes::TypePropertiesIterator
            | TypeDefinitionIdentTypes::ConstComponents
            | TypeDefinitionIdentTypes::ConstExtensions
            | TypeDefinitionIdentTypes::ConstType
            | TypeDefinitionIdentTypes::Type
            | TypeDefinitionIdentTypes::TypeValue => type_definition.type_name().to_string(),
        }
    }
}
