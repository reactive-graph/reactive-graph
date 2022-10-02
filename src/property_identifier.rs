use crate::NAMESPACE_PROPERTY_TYPE;
use indradb::Identifier;
use uuid::Uuid;

/// Safely constructs a property identifier.
///
/// Fallback: generate a UUID v5 based on the property name and a given namespace. The generated
/// property identifier is stable for the property name.
pub fn property_identifier<S: Into<String>>(property_name: S) -> Identifier {
    let property_name = property_name.into();
    Identifier::new(&property_name).unwrap_or_else(|_| Identifier::new(Uuid::new_v5(&NAMESPACE_PROPERTY_TYPE, property_name.as_bytes()).to_string()).unwrap())
}
