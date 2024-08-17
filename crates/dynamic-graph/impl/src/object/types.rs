use convert_case::Case::Camel;
use convert_case::Case::Pascal;
use convert_case::Casing;
use std::fmt::Display;
use std::fmt::Formatter;

use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;

pub struct DynamicGraphTypeDefinition {
    ty: TypeDefinition,
}

impl DynamicGraphTypeDefinition {
    pub fn field_name(&self) -> String {
        self.ty.type_name.to_case(Camel)
    }

    pub fn mutation_field_name(&self, action: &str) -> String {
        format!("{}{}", action, self.ty.type_name.to_case(Pascal))
    }

    pub fn field_name_with_suffix(&self) -> String {
        format!("{}{}", self.field_name(), self.ty.type_id_type.full_name())
    }

    pub fn outbound_type_name(&self) -> String {
        format!("outbound_{}_{}", &self.ty.namespace, &self.ty.type_name)
    }

    pub fn inbound_type_name(&self) -> String {
        format!("inbound_{}_{}", &self.ty.namespace, &self.ty.type_name)
    }

    pub fn mutation_type_name(&self) -> String {
        format!("{}_Mutations", self)
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

impl Display for DynamicGraphTypeDefinition {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}_{}_{}", self.namespace(), self.type_name(), self.ty.type_id_type.full_name())
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
