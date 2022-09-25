enum TypeKind {
    Scalar,
    Object,
    Interface,
    Union,
    Enum,
    InputObject,
    List,
    NonNull,
}

impl ToString for TypeKind {
    fn to_string(&self) -> String {
        match self {
            TypeKind::Scalar => String::from("SCALAR"),
            TypeKind::Object => String::from("OBJECT"),
            TypeKind::Interface => String::from("INTERFACE"),
            TypeKind::Union => String::from("UNION"),
            TypeKind::Enum => String::from("ENUM"),
            TypeKind::InputObject => String::from("INPUT_OBJECT"),
            TypeKind::List => String::from("LIST"),
            TypeKind::NonNull => String::from("NON_NULL"),
        }
    }
}
