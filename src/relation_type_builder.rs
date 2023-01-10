use serde_json::Value;

use crate::model::ComponentOrEntityTypeId;
use crate::model::ComponentTypeId;
use crate::model::DataType;
use crate::model::EntityTypeId;
use crate::model::Extension;
use crate::model::ExtensionTypeId;
use crate::model::PropertyType;
use crate::model::RelationType;
use crate::model::RelationTypeId;

#[allow(dead_code)]
pub struct RelationTypeBuilder {
    outbound_type: ComponentOrEntityTypeId,
    ty: RelationTypeId,
    inbound_type: ComponentOrEntityTypeId,
    description: String,
    components: Vec<ComponentTypeId>,
    properties: Vec<PropertyType>,
    extensions: Vec<Extension>,
}

#[allow(dead_code)]
impl RelationTypeBuilder {
    pub fn new<OT: Into<ComponentOrEntityTypeId>, RT: Into<RelationTypeId>, IT: Into<ComponentOrEntityTypeId>>(
        outbound_type: OT,
        ty: RT,
        inbound_type: IT,
    ) -> RelationTypeBuilder {
        RelationTypeBuilder {
            outbound_type: outbound_type.into(),
            ty: ty.into(),
            inbound_type: inbound_type.into(),
            description: String::new(),
            components: Vec::new(),
            properties: Vec::new(),
            extensions: Vec::new(),
        }
    }

    pub fn new_from_type<S: Into<String>>(
        outbound_namespace: S,
        outbound_type_name: S,
        namespace: S,
        type_name: S,
        inbound_namespace: S,
        inbound_type_name: S,
    ) -> RelationTypeBuilder {
        let outbound_ty = EntityTypeId::new_from_type(outbound_namespace, outbound_type_name);
        let ty = RelationTypeId::new_from_type(namespace, type_name);
        let inbound_ty = EntityTypeId::new_from_type(inbound_namespace, inbound_type_name);
        RelationTypeBuilder::new(outbound_ty, ty, inbound_ty)
    }

    pub fn description<S: Into<String>>(&mut self, description: S) -> &mut RelationTypeBuilder {
        self.description = description.into();
        self
    }

    pub fn component<C: Into<ComponentTypeId>>(&mut self, ty: C) -> &mut RelationTypeBuilder {
        self.components.push(ty.into());
        self
    }

    pub fn component_from_type<S: Into<String>>(&mut self, namespace: S, component_name: S) -> &mut RelationTypeBuilder {
        self.components.push(ComponentTypeId::new_from_type(namespace, component_name));
        self
    }

    pub fn property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), data_type));
        self
    }

    pub fn input_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::input(property_name.into(), data_type));
        self
    }

    pub fn output_property<S: Into<String>>(&mut self, property_name: S, data_type: DataType) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::output(property_name.into(), data_type));
        self
    }

    pub fn property_from<S: Into<PropertyType>>(&mut self, property_type: S) -> &mut RelationTypeBuilder {
        self.properties.push(property_type.into());
        self
    }

    pub fn string_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::String));
        self
    }

    pub fn bool_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Bool));
        self
    }

    pub fn number_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Number));
        self
    }

    pub fn array_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Array));
        self
    }

    pub fn object_property<S: Into<String>>(&mut self, property_name: S) -> &mut RelationTypeBuilder {
        self.properties.push(PropertyType::new(property_name.into(), DataType::Object));
        self
    }

    pub fn extension<S: Into<String>>(&mut self, namespace: S, name: S, extension: Value) -> &mut RelationTypeBuilder {
        let ty = ExtensionTypeId::new_from_type(namespace.into(), name.into());
        self.extensions.push(Extension {
            ty,
            description: Default::default(),
            extension,
        });
        self
    }

    pub fn build(&self) -> RelationType {
        RelationType::new(
            self.outbound_type.clone(),
            self.ty.clone(),
            self.inbound_type.clone(),
            self.description.clone(),
            self.components.to_vec(),
            self.properties.to_vec(),
            self.extensions.to_vec(),
        )
    }
}
