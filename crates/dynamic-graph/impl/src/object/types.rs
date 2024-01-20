use convert_case::Case::Camel;
use convert_case::Case::Pascal;
use convert_case::Casing;

use inexor_rgf_graph::ComponentTypeId;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::NamespacedTypeGetter;
use inexor_rgf_graph::RelationTypeId;
use inexor_rgf_graph::TypeDefinition;
use inexor_rgf_graph::TypeDefinitionGetter;

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
        format!("{}_Mutations", self.to_string())
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
        format!("{}_{}_{}", self.namespace(), self.type_name(), self.ty.type_id_type.full_name())
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
