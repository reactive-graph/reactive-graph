use schemars::JsonSchema;
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::fmt::Formatter;
use typed_builder::TypedBuilder;

use crate::NamespacedType;
use crate::NamespacedTypeGetter;
use crate::TypeIdType;
use crate::TYPE_ID_TYPE_SEPARATOR;

/// Definition of a type with the type of the type, the namespace and the name of the type.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema, TypedBuilder)]
pub struct TypeDefinition {
    pub type_id_type: TypeIdType,
    pub namespace: String,
    pub type_name: String,
}

impl TypeDefinition {
    /// Constructs a new type definition from the given type of types and the given namespaced type.
    pub fn new(type_type: TypeIdType, nt: NamespacedType) -> TypeDefinition {
        TypeDefinition {
            type_id_type: type_type,
            namespace: nt.namespace,
            type_name: nt.type_name,
        }
    }

    /// Constructs a type definition from the given type of types, the given namespace and type name.
    pub fn new_from_type<S: Into<String>>(type_type: TypeIdType, namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition {
            type_id_type: type_type,
            namespace: namespace.into(),
            type_name: type_name.into(),
        }
    }

    /// Constructs a type definition for a component.
    pub fn component<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::Component, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a entity type.
    pub fn entity_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::EntityType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a relation type.
    pub fn relation_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::RelationType, NamespacedType::new(namespace, type_name))
    }

    /// Constructs a type definition for a flow type.
    pub fn flow_type<S: Into<String>>(namespace: S, type_name: S) -> TypeDefinition {
        TypeDefinition::new(TypeIdType::FlowType, NamespacedType::new(namespace, type_name))
    }
}

impl NamespacedTypeGetter for TypeDefinition {
    fn namespace(&self) -> String {
        self.namespace.clone()
    }

    fn type_name(&self) -> String {
        self.type_name.clone()
    }
}

/// Returns the fully qualified type name.
impl Display for TypeDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}{}{}",
            self.type_id_type.to_string(),
            &TYPE_ID_TYPE_SEPARATOR,
            &self.namespace,
            &TYPE_ID_TYPE_SEPARATOR,
            &self.type_name
        )
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for TypeIdType {
    fn from(type_definition: &TypeDefinition) -> Self {
        type_definition.type_id_type
    }
}

/// Returns the type of the type.
impl From<&TypeDefinition> for NamespacedType {
    fn from(type_definition: &TypeDefinition) -> Self {
        NamespacedType {
            namespace: type_definition.namespace.clone(),
            type_name: type_definition.type_name.clone(),
        }
    }
}

impl TryFrom<&String> for TypeDefinition {
    type Error = ();

    fn try_from(s: &String) -> Result<Self, Self::Error> {
        let mut s = s.split(&TYPE_ID_TYPE_SEPARATOR);
        let type_type: TypeIdType = s.next().ok_or(())?.try_into()?;

        let namespace = s.next().ok_or(())?;
        if namespace.is_empty() {
            return Err(());
        }
        let type_name = s.next().ok_or(())?;
        if type_name.is_empty() {
            return Err(());
        }
        if s.next().is_some() {
            return Err(());
        }
        let td = TypeDefinition::new(type_type, NamespacedType::new(namespace, type_name));
        Ok(td)
    }
}

/// Grants access to the type definition of a type of types.
pub trait TypeDefinitionGetter {
    /// Returns the type definition of the type.
    fn type_definition(&self) -> TypeDefinition;
}

#[cfg(test)]
mod tests {
    use schemars::schema_for;

    use crate::NamespacedType;
    use crate::NamespacedTypeGetter;
    use crate::TypeDefinition;
    use crate::TypeIdType;
    use reactive_graph_test_utils::r_string;

    #[test]
    fn type_definition_component_test() {
        let namespace = r_string();
        let type_name = r_string();
        let nt = NamespacedType::new(&namespace, &type_name);
        let td = TypeDefinition::new(TypeIdType::Component, nt.clone());
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::Component, tid);

        let nt2: NamespacedType = NamespacedType::from(&td);
        assert_eq!(nt, nt2);

        // let t = String::from(&td);
        // assert_eq!(format!("c__{namespace}__{type_name}"), t.as_str());
        let t = format!("c__{namespace}__{type_name}");

        let td2 = TypeDefinition::try_from(&t).unwrap();
        assert_eq!(TypeIdType::Component, td2.type_id_type);
        assert_eq!(namespace, td2.namespace());
        assert_eq!(type_name, td2.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), td2.to_string());
        assert_eq!(td, td2);
    }

    #[test]
    fn type_definition_component_2_test() {
        let namespace = r_string();
        let type_name = r_string();
        let td = TypeDefinition::new_from_type(TypeIdType::Component, &namespace, &type_name);
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
    }

    #[test]
    fn type_definition_component_3_test() {
        let namespace = r_string();
        let type_name = r_string();
        let td = TypeDefinition::component(&namespace, &type_name);
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
    }

    #[test]
    fn type_definition_entity_type_test() {
        let namespace = r_string();
        let type_name = r_string();
        let td = TypeDefinition::entity_type(&namespace, &type_name);
        assert_eq!(TypeIdType::EntityType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("e__{namespace}__{type_name}"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::EntityType, tid);
    }

    #[test]
    fn type_definition_relation_type_test() {
        let namespace = r_string();
        let type_name = r_string();
        let td = TypeDefinition::relation_type(&namespace, &type_name);
        assert_eq!(TypeIdType::RelationType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("r__{namespace}__{type_name}"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::RelationType, tid);
    }

    #[test]
    fn type_definition_flow_type_test() {
        let namespace = r_string();
        let type_name = r_string();
        let td = TypeDefinition::flow_type(&namespace, &type_name);
        assert_eq!(TypeIdType::FlowType, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("f__{namespace}__{type_name}"), td.to_string());

        let tid: TypeIdType = TypeIdType::from(&td);
        assert_eq!(TypeIdType::FlowType, tid);
    }

    #[test]
    fn type_definition_component_from_string_test() {
        let namespace = r_string();
        let type_name = r_string();
        let t = format!("c__{namespace}__{type_name}");
        let td = TypeDefinition::try_from(&t).unwrap();
        assert_eq!(TypeIdType::Component, td.type_id_type);
        assert_eq!(namespace, td.namespace());
        assert_eq!(type_name, td.type_name());
        assert_eq!(format!("c__{namespace}__{type_name}"), td.to_string());
    }

    #[test]
    fn type_definition_json_schema() {
        let schema = schema_for!(TypeDefinition);
        println!("{}", serde_json::to_string_pretty(&schema).unwrap());
    }
}
