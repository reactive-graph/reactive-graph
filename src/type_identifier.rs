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

/// Returns the namespace and type name from the given identifier.
pub fn get_namespace_and_type_name(t: &Identifier) -> (String, String) {
    let type_name = t.to_string();
    match type_name.split_once("__") {
        Some((namespace, type_name)) => (namespace.to_string(), type_name.to_string()),
        None => (String::new(), type_name),
    }
}
