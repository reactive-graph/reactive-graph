use crate::field::create_properties_from_field_arguments;
use crate::field::to_field_value;
use crate::field::to_input_type_ref;
use crate::field::to_type_ref;
use crate::interface::flow::INTERFACE_FLOW_FIELD_ID;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::Error;
use async_graphql::ID;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::FieldValue;
use async_graphql::dynamic::InputValue;
use async_graphql::dynamic::ResolverContext;
use async_graphql::dynamic::TypeRef;
use reactive_graph_dynamic_graph_api::FlowInstanceIsNotOfType;
use reactive_graph_dynamic_graph_api::FlowInstanceNotFound;
use reactive_graph_graph::DataType;
use reactive_graph_graph::FlowType;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::TypeDefinitionGetter;
use reactive_graph_reactive_model_impl::ReactiveFlow;
use reactive_graph_reactive_service_api::ReactiveFlowManager;
use reactive_graph_runtime_model::LabeledProperties::LABEL;
use reactive_graph_type_system_api::EntityTypeManager;
use serde_json::Value;
use std::str::FromStr;
use std::sync::Arc;
use uuid::Uuid;

pub fn flow_id_field() -> Field {
    Field::new(INTERFACE_FLOW_FIELD_ID, TypeRef::named_nn(TypeRef::ID), |ctx| {
        FieldFuture::new(async move {
            let flow_instance = ctx.parent_value.try_downcast_ref::<ReactiveFlow>()?;
            Ok(Some(FieldValue::value(ID(flow_instance.id.to_string()))))
        })
    })
}

pub fn flow_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let flow_instance = ctx.parent_value.try_downcast_ref::<ReactiveFlow>()?;
            Ok(flow_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}

pub fn flow_query_field(flow_type: &FlowType) -> Field {
    let ty = flow_type.ty.clone();
    let flow_type_inner = flow_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn_list_nn(dy_ty.to_string()), move |ctx| {
        let ty = ty.clone();
        let flow_type = flow_type_inner.clone();
        let entity_ty = flow_type.wrapper_type();
        FieldFuture::new(async move {
            let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let flow_instance = flow_instance_manager.get(id).ok_or(Error::new("Uuid not found"))?;
                if flow_instance.ty != entity_ty {
                    return Err(Error::new(format!("Flow {} is not a {}", id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(flow_instance)])));
            }
            if let Ok(label) = ctx.args.try_get("label") {
                let flow_instance = flow_instance_manager.get_by_label(label.string()?).ok_or(Error::new("Label not found"))?;
                if flow_instance.ty != entity_ty {
                    return Err(Error::new(format!("Flow {} is not a {}", flow_instance.id, &ty)));
                }
                return Ok(Some(FieldValue::list(vec![FieldValue::owned_any(flow_instance)])));
            }
            let instances = get_flow_instances_by_type_filter_by_properties(&ctx, &flow_type)?;
            Ok(Some(FieldValue::list(instances.into_iter().map(FieldValue::owned_any))))
        })
    })
    .description(flow_type.description.clone())
    .argument(InputValue::new("id", TypeRef::named(TypeRef::STRING)))
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_flow_type_variables_as_field_arguments(field, flow_type, true, true);
    field
}

// , flow_instance_manager: Arc<dyn ReactiveFlowManager + Send + Sync>
pub fn flow_creation_field(flow_type: &FlowType) -> Option<Field> {
    let flow_type_inner = flow_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let mut field = Field::new(dy_ty.mutation_field_name("create"), TypeRef::named_nn(dy_ty.to_string()), move |ctx| {
        let ty = flow_type_inner.ty.clone();
        let flow_type = flow_type_inner.clone();
        FieldFuture::new(async move {
            let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
            let entity_type_manager = ctx.data::<Arc<dyn EntityTypeManager + Send + Sync>>()?;

            let entity_ty = flow_type.wrapper_type();
            let Some(entity_type) = entity_type_manager.get(&entity_ty) else {
                return Err(Error::new(format!("Missing entity type {}", entity_ty.type_definition())));
            };
            let id = ctx.args.get("id").and_then(|id| id.string().ok().and_then(|s| Uuid::from_str(s).ok()));
            if let Some(id) = id {
                if flow_instance_manager.has(id) {
                    return Err(Error::new(format!("Uuid {id} is already taken")));
                }
            }

            // let id = if let Some(id) = ctx.args.get("id") {
            //     let id = Uuid::from_str(id.string()?)?;
            //     if flow_instance_manager.has(id) {
            //         return Err(Error::new(format!("Uuid {} is already taken", id)));
            //     }
            //     Some(id)
            // } else {
            //     None
            // };
            let properties = create_properties_from_field_arguments(&ctx, &entity_type.properties)?;
            // let properties = ReactiveProperties::new_with_id_from_properties(id, properties);

            let variables = create_properties_from_field_arguments(&ctx, &flow_type.variables)?;
            // let variables = ReactiveProperties::new_with_id_from_properties(id, variables);

            match flow_instance_manager.create_from_type(&ty, id, variables, properties) {
                Ok(reactive_flow) => Ok(Some(FieldValue::owned_any(reactive_flow))),
                Err(e) => Err(Error::new(format!("Failed to create reactive flow: {e:?}"))),
            }
            // let Ok(reactive_flow) = flow_instance_manager.create_from_type(&ty, variables, properties) else {
            //     return Err(Error::new(format!("Failed to create reactive flow: {}",)));
            // };
            //
            // let reactive_flow = ReactiveFlow::builder().ty(&ty).id(id).properties(properties).build();
            // // TODO: flow_instance_manager.create
            // let x = flow_instance_manager.register_flow_instance_and_reactive_instances(reactive_flow);
            // if let Ok(reactive_flow) = flow_instance_manager.register_reactive_instance(reactive_flow) {
            //     return Ok(Some(FieldValue::owned_any(reactive_flow)));
            // }
            // Ok(None)
        })
    })
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)));
    field = add_flow_type_variables_as_field_arguments(field, flow_type, false, false);
    Some(field)
}

pub fn flow_mutation_field(flow_type: &FlowType) -> Option<Field> {
    let ty = flow_type.ty.clone();
    let flow_type_inner = flow_type.clone();
    let dy_ty = DynamicGraphTypeDefinition::from(&flow_type.ty);
    let mut field = Field::new(dy_ty.field_name(), TypeRef::named_nn(dy_ty.mutation_type_name()), move |ctx| {
        let ty = ty.clone();
        let flow_type = flow_type_inner.clone();
        FieldFuture::new(async move {
            let flow_instance_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
            // Multiple ids
            if let Ok(ids) = ctx.args.try_get("ids") {
                let mut flow_instances = Vec::new();
                for id in ids
                    .list()?
                    .iter()
                    .filter_map(|id| id.string().map(str::to_string).ok())
                    .filter_map(|id| Uuid::from_str(&id).ok())
                {
                    if let Some(flow_instance) = flow_instance_manager.get(id) {
                        if flow_instance.ty != flow_type.wrapper_type() {
                            return Err(FlowInstanceIsNotOfType(id, ty.clone(), flow_type.wrapper_type()).into());
                        }
                        flow_instances.push(flow_instance);
                    }
                }
                let field_value = FieldValue::owned_any(flow_instances);
                return Ok(Some(field_value));
            }
            // Single ids
            if let Ok(id) = ctx.args.try_get("id") {
                let id = Uuid::from_str(id.string()?)?;
                let flow_instance = flow_instance_manager.get(id).ok_or(FlowInstanceNotFound(id))?;

                if flow_instance.ty != flow_type.wrapper_type() {
                    return Err(FlowInstanceIsNotOfType(id, ty.clone(), flow_type.wrapper_type()).into());
                }
                let flow_instances = vec![flow_instance];
                let field_value = FieldValue::owned_any(flow_instances);
                return Ok(Some(field_value));
            }
            // TODO: implement label matching
            let instances = get_flow_instances_by_type_filter_by_properties(&ctx, &flow_type)?;
            let field_value = FieldValue::owned_any(instances);
            Ok(Some(field_value))
        })
    })
    .description(flow_type.description.clone())
    .argument(InputValue::new("ids", TypeRef::named_nn_list(TypeRef::ID)))
    .argument(InputValue::new("id", TypeRef::named(TypeRef::ID)))
    // TODO: implement label matching
    .argument(InputValue::new("label", TypeRef::named(TypeRef::STRING)));
    field = add_flow_type_variables_as_field_arguments(field, flow_type, true, true);
    Some(field)
}

fn get_flow_instances_by_type_filter_by_properties(ctx: &ResolverContext, flow_type: &FlowType) -> async_graphql::Result<Vec<ReactiveFlow>> {
    let reactive_flow_manager = ctx.data::<Arc<dyn ReactiveFlowManager + Send + Sync>>()?;
    let mut instances = reactive_flow_manager.get_by_type(&flow_type.ty);
    for property in flow_type.variables.iter() {
        let Some(expected_value) = ctx.args.get(&property.name) else {
            continue;
        };
        instances.retain(|instance| match instance.get(&property.name) {
            Some(actual_value) => match &property.data_type {
                DataType::Null => false,
                DataType::Bool => expected_value
                    .boolean()
                    .map(|expected_value| actual_value.as_bool().map(|actual_value| expected_value == actual_value).unwrap_or(false))
                    .unwrap_or(false),
                DataType::Number => {
                    if let Ok(expected_value) = expected_value.i64() {
                        actual_value.as_i64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else if let Ok(expected_value) = expected_value.u64() {
                        actual_value.as_u64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else if let Ok(expected_value) = expected_value.f64() {
                        actual_value.as_f64().map(|actual_value| expected_value == actual_value).unwrap_or(false)
                    } else {
                        false
                    }
                }
                DataType::String => expected_value
                    .string()
                    .map(|expected_value| actual_value.as_str().map(|actual_value| expected_value == actual_value).unwrap_or(false))
                    .unwrap_or(false),
                DataType::Array => {
                    if let Ok(_l) = expected_value.list() {
                        if let Ok(expected_value) = expected_value.deserialize::<Value>() {
                            if expected_value.is_array() && actual_value.is_array() {
                                expected_value == actual_value
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                DataType::Object => {
                    if let Ok(_o) = expected_value.object() {
                        if let Ok(expected_value) = expected_value.deserialize::<Value>() {
                            if expected_value.is_object() && actual_value.is_object() {
                                expected_value == actual_value
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
                DataType::Any => match expected_value.deserialize::<Value>() {
                    Ok(expected_value) => expected_value == actual_value,
                    Err(_) => false,
                },
            },
            None => false,
        });
    }
    Ok(instances)
}

fn add_flow_type_variables_as_field_arguments(mut field: Field, flow_type: &FlowType, is_optional: bool, exclude_label: bool) -> Field {
    for property in flow_type.variables.iter() {
        if exclude_label && property.name == LABEL.property_name() {
            continue;
        }
        if let Some(type_ref) = to_input_type_ref(property.value(), is_optional) {
            field = field.argument(InputValue::new(&property.name, type_ref));
        }
    }
    field
}
