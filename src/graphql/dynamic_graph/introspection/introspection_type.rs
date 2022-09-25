use apollo_compiler::values::Type;

pub const INTROSPECTION_SCHEMA: &str = "__Schema";
pub const INTROSPECTION_TYPE: &str = "__Type";
pub const INTROSPECTION_TYPE_KIND: &str = "__TypeKind";
pub const INTROSPECTION_FIELD: &str = "__Field";
pub const INTROSPECTION_INPUT_VALUE: &str = "__InputValue";
pub const INTROSPECTION_ENUM_VALUE: &str = "__EnumValue";
pub const INTROSPECTION_DIRECTIVE: &str = "__Directive";
pub const INTROSPECTION_DIRECTIVE_LOCATION: &str = "__DirectiveLocation";

pub enum IntrospectionType {
    Schema,
    Type,
    TypeKind,
    Field,
    InputValue,
    EnumValue,
    Directive,
    DirectiveLocation,
}

impl ToString for IntrospectionType {
    fn to_string(&self) -> String {
        match self {
            IntrospectionType::Schema => String::from(INTROSPECTION_SCHEMA),
            IntrospectionType::Type => String::from(INTROSPECTION_TYPE),
            IntrospectionType::TypeKind => String::from(INTROSPECTION_TYPE_KIND),
            IntrospectionType::Field => String::from(INTROSPECTION_FIELD),
            IntrospectionType::InputValue => String::from(INTROSPECTION_INPUT_VALUE),
            IntrospectionType::EnumValue => String::from(INTROSPECTION_ENUM_VALUE),
            IntrospectionType::Directive => String::from(INTROSPECTION_DIRECTIVE),
            IntrospectionType::DirectiveLocation => String::from(INTROSPECTION_DIRECTIVE_LOCATION),
        }
    }
}

impl TryFrom<&String> for IntrospectionType {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        match value.as_str() {
            INTROSPECTION_SCHEMA => Ok(IntrospectionType::Schema),
            INTROSPECTION_TYPE => Ok(IntrospectionType::Type),
            INTROSPECTION_TYPE_KIND => Ok(IntrospectionType::TypeKind),
            INTROSPECTION_FIELD => Ok(IntrospectionType::Field),
            INTROSPECTION_INPUT_VALUE => Ok(IntrospectionType::InputValue),
            INTROSPECTION_ENUM_VALUE => Ok(IntrospectionType::EnumValue),
            INTROSPECTION_DIRECTIVE => Ok(IntrospectionType::Directive),
            INTROSPECTION_DIRECTIVE_LOCATION => Ok(IntrospectionType::DirectiveLocation),
            _ => Err(()),
        }
    }
}

impl TryFrom<&Type> for IntrospectionType {
    type Error = ();

    fn try_from(ty: &Type) -> Result<Self, Self::Error> {
        IntrospectionType::try_from(&ty.name())
    }
}

pub(crate) fn is_introspection_type(ty: &Type) -> bool {
    IntrospectionType::try_from(ty).is_ok()
}

pub enum ResolverType {
    IntrospectionType(IntrospectionType),
    Other,
}

impl From<&Type> for ResolverType {
    fn from(ty: &Type) -> Self {
        if let Ok(introspection_type) = ty.try_into() {
            return ResolverType::IntrospectionType(introspection_type);
        }
        ResolverType::Other
    }
}
