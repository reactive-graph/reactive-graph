use crate::field::namespace_path_field;
use crate::field::property::property_container_update_field_arguments;
use crate::field::property::property_container_update_properties;
use crate::field::property::property_container_validate_input_fields;
use crate::field::update::FIELD_NAME_UPDATE;
use crate::object_type_name::object_type_name;
use crate::object_type_name::object_type_ref;
use async_graphql::Error;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::Object;
use async_trait::async_trait;
use log::trace;
use reactive_graph_dynamic_graph_api::ComponentMutationObjectFactory;
use reactive_graph_dynamic_graph_api::FIELD_NAME_JSON_SCHEMA;
use reactive_graph_dynamic_graph_api::JsonSchemaFieldFactory;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::Component;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_lifecycle::Lifecycle;
use reactive_graph_reactive_model_api::ReactiveInstanceUnidentifiable;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveRelation;
use reactive_graph_type_system_api::ComponentManager;
use springtime_di::Component;
use springtime_di::component_alias;
use std::sync::Arc;

#[derive(Component)]
pub struct ComponentMutationObjectFactoryImpl {
    component_manager: Arc<dyn ComponentManager + Send + Sync>,
    json_schema_field_factory: Arc<dyn JsonSchemaFieldFactory + Send + Sync>,
}

#[async_trait]
#[component_alias]
impl ComponentMutationObjectFactory for ComponentMutationObjectFactoryImpl {
    fn create_mutation_objects(&self) -> Vec<Object> {
        let mut objects = Vec::new();
        for (_, component) in self.component_manager.get_all() {
            objects.push(self.create_mutation_object(component));
        }
        objects
    }

    fn create_mutation_object(&self, component: Component) -> Object {
        let object_type_name = object_type_name(&component.ty, RootObjectType::Mutation);
        trace!("Create mutation object {object_type_name} for {}", &component.ty);
        let mut object = Object::new(object_type_name)
            .description(&component.description)
            .field(namespace_path_field(component.namespace()));
        if let Some(field) = self
            .json_schema_field_factory
            .get_json_schema_field(FIELD_NAME_JSON_SCHEMA, &component.type_definition())
        {
            object = object.field(field);
        }
        if let Some(update_field) = self.create_update_field(&component) {
            object = object.field(update_field);
        }
        object
    }

    fn create_update_field(&self, component: &Component) -> Option<Field> {
        let component_inner = component.clone();
        let update_field = Field::new(FIELD_NAME_UPDATE, object_type_ref(&component.ty, RootObjectType::Interface), move |ctx| {
            let component = component_inner.clone();
            FieldFuture::new(async move {
                trace!("{:?}", ctx.parent_value);
                if let Some(reactive_entity) = ctx.parent_value.downcast_ref::<ReactiveEntity>() {
                    trace!("Resolved reactive entity using type bound");
                    // First validate all input fields for mutability and correct datatype
                    property_container_validate_input_fields(&component, &ctx)?;
                    // Set properties
                    property_container_update_properties(&component, reactive_entity, &ctx)?;
                    return Ok(Some(FieldValue::owned_any(reactive_entity.clone())));
                }
                if let Some(reactive_relation) = ctx.parent_value.downcast_ref::<ReactiveRelation>() {
                    trace!("Resolved reactive relation using type bound");
                    // First validate all input fields for mutability and correct datatype
                    property_container_validate_input_fields(&component, &ctx)?;
                    // Set properties
                    property_container_update_properties(&component, reactive_relation, &ctx)?;
                    return Ok(Some(FieldValue::owned_any(reactive_relation.clone())));
                }
                if let Some(reactive_instance) = ctx.parent_value.downcast_ref::<Arc<Box<dyn ReactiveInstanceUnidentifiable>>>() {
                    trace!("Resolved reactive instance using downcast to ReactiveInstanceUnidentifiable");
                    // First validate all input fields for mutability and correct datatype
                    property_container_validate_input_fields(&component, &ctx)?;
                    // Set properties
                    property_container_update_properties(&component, reactive_instance.as_ref().as_ref(), &ctx)?;
                    return Ok(Some(FieldValue::owned_any(reactive_instance.clone())));
                }
                // match ctx.parent_value.0 {
                //     FieldValueInner::Value(_) => {}
                //     FieldValueInner::BorrowedAny(_, _) => {}
                //     FieldValueInner::OwnedAny(_, _) => {}
                //     FieldValueInner::List(_) => {}
                //     FieldValueInner::WithType { .. } => {}
                // }
                return Err(Error::new(format!("Could not update field: Field value: {:?}", ctx.parent_value)));
                // let reactive_instance = ctx.parent_value.try_downcast_ref::<Arc<Box<dyn ReactiveInstanceUnidentifiable>>>()?;
                // // First validate all input fields for mutability and correct datatype
                // property_container_validate_input_fields(&component, &ctx)?;
                // // Set properties
                // property_container_update_properties(&component, reactive_instance.as_ref().as_ref(), &ctx)?;
                // Ok(Some(FieldValue::owned_any(reactive_instance.clone())))
                // let reactive_instances = ctx.parent_value.try_downcast_ref::<Vec<Arc<Box<dyn ReactiveInstanceUnidentifiable>>>>()?;
                // for reactive_instance in reactive_instances {
                //     // First validate all input fields for mutability and correct datatype
                //     property_container_validate_input_fields(&component, &ctx)?;
                //     // Set properties
                //     property_container_update_properties(&component, reactive_instance.as_ref().as_ref(), &ctx)?;
                // }
                // Ok(Some(FieldValue::list(
                //     reactive_instances
                //         .iter()
                //         .map(|reactive_relation| FieldValue::owned_any(reactive_relation.clone())),
                // )))
            })
        })
        .description("Updates the entity instance");
        property_container_update_field_arguments(component, update_field)
    }
}

#[async_trait]
impl Lifecycle for ComponentMutationObjectFactoryImpl {}
