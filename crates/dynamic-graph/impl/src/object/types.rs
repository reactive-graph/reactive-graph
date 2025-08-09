use convert_case::Case::Camel;
use convert_case::Case::Pascal;
use convert_case::Casing;
use log::error;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::FlowTypeId;
use reactive_graph_graph::Namespace;
use reactive_graph_graph::NamespaceSegment;
use reactive_graph_graph::NamespacedType;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::RelationTypeId;
use reactive_graph_graph::TypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_graph::TypeIdType;
use std::fmt::Display;
use std::fmt::Formatter;

pub struct DynamicGraphTypeDefinition {
    ty: TypeDefinition,
}

impl DynamicGraphTypeDefinition {
    pub fn field_name(&self) -> String {
        self.ty.type_name.to_case(Camel)
    }

    pub fn field_name_with_appendix(&self, appendix: &str) -> String {
        format!("{}{}{}", self.ty.type_name.to_case(Camel), self.ty.type_id_type.full_name().to_case(Pascal), appendix)
    }

    pub fn field_name_with_namespace_type_type_and_appendix(&self, appendix: &str) -> String {
        let field_name = format!(
            "{}_{}{}{}",
            self.ty.namespace.to_case(Pascal),
            self.ty.type_name.to_case(Pascal),
            self.ty.type_id_type.full_name().to_case(Pascal),
            appendix
        );
        error!("{field_name}");
        field_name
    }

    pub fn mutation_field_name(&self, action: &str) -> String {
        format!("{}{}", action, self.ty.type_name.to_case(Pascal))
    }

    pub fn mutation_field_name_with_appendix(&self, action: &str, appendix: &str) -> String {
        format!(
            "{}{}{}{}",
            action,
            self.ty.type_name.to_case(Pascal),
            self.ty.type_id_type.full_name().to_case(Pascal),
            appendix
        )
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
        format!("{self}_Mutations")
    }
}

impl NamespacedTypeGetter for DynamicGraphTypeDefinition {
    fn namespaced_type(&self) -> NamespacedType {
        self.ty.namespaced_type()
    }

    // fn namespace(&self) -> Namespace {
    //     self.ty.namespace.to_case(Pascal)
    // }

    fn namespace(&self) -> Namespace {
        self.ty.namespace()
    }

    fn path(&self) -> Namespace {
        self.ty.path()
    }

    fn type_name(&self) -> NamespaceSegment {
        self.ty.type_name()
    }

    // fn type_name(&self) -> String {
    //     self.ty.type_name.to_case(Pascal)
    // }
}

impl TypeDefinitionGetter for DynamicGraphTypeDefinition {
    fn type_definition(&self) -> TypeDefinition {
        self.ty.clone()
    }

    // fn type_id_type() -> TypeIdType {
    //     self.ty.
    //
    // }
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

impl From<&FlowTypeId> for DynamicGraphTypeDefinition {
    fn from(ty: &FlowTypeId) -> Self {
        DynamicGraphTypeDefinition { ty: ty.type_definition() }
    }
}
