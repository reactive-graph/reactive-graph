use indradb::Identifier;
use uuid::Uuid;

/// Safely constructs a type identifier.
///
/// Fallback: generate a UUID v5 based on the type_name and a given namespace. The generated
/// type identifier is stable for the same namespace, name and type_namespace.
pub fn fully_qualified_identifier(namespace: &str, name: &str, type_namespace: &Uuid) -> Identifier {
    let fully_qualified_name = format!("{namespace}__{name}");
    Identifier::new(fully_qualified_name.as_str())
        .unwrap_or_else(|_| Identifier::new(Uuid::new_v5(type_namespace, fully_qualified_name.as_bytes()).to_string()).unwrap())
}
