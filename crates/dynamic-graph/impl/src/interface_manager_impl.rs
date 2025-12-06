use crate::extension::divergent::is_divergent;
use crate::field::NAMESPACE_FIELD_NAME;
use crate::field::property::datatype::to_type_ref;
use crate::object_type_name::object_type_name;
use crate::type_ref::TYPE_REF_ID;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Interface;
use async_graphql::dynamic::InterfaceField;
use async_graphql::dynamic::Object;
use async_graphql::dynamic::TypeRef;
use async_trait::async_trait;
use log::info;
use log::trace;
use reactive_graph_dynamic_graph_api::INTERFACE_ENTITY;
use reactive_graph_dynamic_graph_api::INTERFACE_ENTITY_FIELD_ID;
use reactive_graph_dynamic_graph_api::INTERFACE_FLOW;
use reactive_graph_dynamic_graph_api::INTERFACE_FLOW_FIELD_ID;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION_FIELD_ID;
use reactive_graph_dynamic_graph_api::INTERFACE_RELATION_FIELD_INSTANCE_ID;
use reactive_graph_dynamic_graph_api::InterfaceManager;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Component;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::ComponentTypeIdContainer;
use reactive_graph_graph::ExtensionContainer;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_type_system_api::ComponentManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct InterfaceManagerImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
}

impl InterfaceManagerImpl {}

#[async_trait]
#[component_alias]
impl InterfaceManager for InterfaceManagerImpl {
    fn get_interfaces(&self) -> Vec<Interface> {
        let mut interfaces = Vec::new();
        for (_, component) in self.component_manager.get_all().into_iter() {
            interfaces.push(self.get_component_interface(component));
        }
        interfaces.push(self.get_entity_interface());
        interfaces.push(self.get_relation_interface());
        interfaces.push(self.get_flow_interface());
        interfaces
    }

    fn get_component_interface(&self, component: Component) -> Interface {
        let interface_type_name = object_type_name(&component.ty, RootObjectType::Interface);
        info!("Create interface {interface_type_name} for {}", &component.ty);
        let mut interface = Interface::new(interface_type_name)
            .description(&component.description)
            .field(component_id_field(&component.ty));
        for field in component.properties.iter() {
            interface = interface.field(property_type_interface_field(field.value()));
        }
        interface
    }

    fn get_entity_interface(&self) -> Interface {
        info!("Create interface {INTERFACE_ENTITY}");
        Interface::new(INTERFACE_ENTITY)
            .description("Entities have outbound and inbound relations as well as components and properties ")
            .field(InterfaceField::new(NAMESPACE_FIELD_NAME, TYPE_REF_ID.clone()))
            .field(InterfaceField::new(INTERFACE_ENTITY_FIELD_ID, TYPE_REF_ID.clone()))
    }

    fn get_relation_interface(&self) -> Interface {
        info!("Create interface {INTERFACE_RELATION}");
        Interface::new(INTERFACE_RELATION)
            .description("Relations have a outbound entity and a inbound entity as well as components and properties,")
            .field(InterfaceField::new(NAMESPACE_FIELD_NAME, TYPE_REF_ID.clone()))
            .field(InterfaceField::new(INTERFACE_RELATION_FIELD_ID, TYPE_REF_ID.clone()))
            .field(InterfaceField::new(INTERFACE_RELATION_FIELD_INSTANCE_ID, TYPE_REF_ID.clone()))
    }

    fn get_flow_interface(&self) -> Interface {
        info!("Create interface {INTERFACE_FLOW}");
        Interface::new(INTERFACE_FLOW)
            .description("Flows have entities and relations.")
            .field(InterfaceField::new(NAMESPACE_FIELD_NAME, TYPE_REF_ID.clone()))
            .field(InterfaceField::new(INTERFACE_FLOW_FIELD_ID, TYPE_REF_ID.clone()))
    }
}

#[async_trait]
impl Lifecycle for InterfaceManagerImpl {}

pub fn component_id_field_name(ty: &ComponentTypeId) -> String {
    format!("_{}", ty.fully_qualified_type_name())
}

/// Field with a unique name on each component interface which allows to identify the
/// components of a type.
pub fn component_id_field(ty: &ComponentTypeId) -> InterfaceField {
    InterfaceField::new(component_id_field_name(ty), TypeRef::named(TypeRef::ID))
}

pub fn instance_component_id_field(ty: &ComponentTypeId) -> Field {
    let ty_inner = ty.clone();
    Field::new(component_id_field_name(ty), TypeRef::named(TypeRef::ID), move |_ctx| {
        let ty = ty_inner.clone();
        FieldFuture::new(async move {
            // Contains the namespace
            Ok(Some(FieldValue::value(ID(ty.namespace().to_string()))))
        })
    })
}

/// Add component identity field to each component and let the `ComponentTypeIdContainer`
/// object implement the component interface.
pub fn component_type_id_container_component_fields<T: ComponentTypeIdContainer + ExtensionContainer + TypeDefinitionGetter>(
    component_type_id_container: &T,
    mut component_type_id_container_object: Object,
) -> Object {
    for component_ty in component_type_id_container.get_components_cloned().iter() {
        // Identity field
        component_type_id_container_object = component_type_id_container_object.field(
            instance_component_id_field(component_ty.key())
                .description(format!("The fully qualified namespace to the component: {}", component_ty.namespace().to_string())),
        );
        if !is_divergent(component_type_id_container, component_ty.key()) {
            // Let the type object implement the component interface
            let interface_type_name = object_type_name(component_ty.key(), RootObjectType::Interface);
            trace!(
                "{} is a {} implements interface {interface_type_name}",
                component_type_id_container.type_definition(),
                component_ty.key()
            );
            component_type_id_container_object = component_type_id_container_object.implement(interface_type_name);
        }
    }
    component_type_id_container_object
}

/// Constructs a field for an interface from a property type.
pub fn property_type_interface_field(property_type: &PropertyType) -> InterfaceField {
    InterfaceField::new(&property_type.name, to_type_ref(&property_type.data_type)).description(&property_type.description)
}
