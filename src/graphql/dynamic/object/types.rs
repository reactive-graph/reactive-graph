use convert_case::Case::Pascal;
use convert_case::Casing;

use crate::model::ComponentTypeId;
use crate::model::EntityTypeId;
use crate::model::NamespacedTypeGetter;
use crate::model::RelationTypeId;
use crate::model::TypeDefinition;
use crate::model::TypeDefinitionGetter;
use crate::model::TypeIdType;

pub struct DynamicGraphTypeDefinition {
    ty: TypeDefinition,
}

impl DynamicGraphTypeDefinition {
    pub fn type_name_with_suffix(&self) -> String {
        format!("{}{}", self.type_name(), self.type_id_suffix())
    }

    pub fn type_id_suffix(&self) -> String {
        match self.ty.type_id_type {
            TypeIdType::Behaviour => "Behaviour",
            TypeIdType::Component => "Component",
            TypeIdType::EntityType => "Entity",
            TypeIdType::RelationType => "Relation",
            TypeIdType::FlowType => "Flow",
        }
        .to_string()
    }

    pub fn outbound_type_name(&self) -> String {
        format!("outbound_{}_{}", &self.ty.namespace, &self.ty.type_name)
    }

    pub fn inbound_type_name(&self) -> String {
        format!("inbound_{}_{}", &self.ty.namespace, &self.ty.type_name)
    }
}

impl NamespacedTypeGetter for DynamicGraphTypeDefinition {
    fn namespace(&self) -> String {
        self.ty.namespace.to_case(Pascal)
    }

    fn type_name(&self) -> String {
        self.ty.type_name.to_case(Pascal)
    }
}

impl ToString for DynamicGraphTypeDefinition {
    fn to_string(&self) -> String {
        format!("{}{}{}", self.namespace(), self.type_name(), self.type_id_suffix())
    }
}

impl From<&ComponentTypeId> for DynamicGraphTypeDefinition {
    fn from(ty: &ComponentTypeId) -> Self {
        DynamicGraphTypeDefinition { ty: ty.type_definition() }
    }
}

impl From<&EntityTypeId> for DynamicGraphTypeDefinition {
    fn from(ty: &EntityTypeId) -> Self {
        DynamicGraphTypeDefinition { ty: ty.type_definition() }
    }
}

impl From<&RelationTypeId> for DynamicGraphTypeDefinition {
    fn from(ty: &RelationTypeId) -> Self {
        DynamicGraphTypeDefinition { ty: ty.type_definition() }
    }
}
