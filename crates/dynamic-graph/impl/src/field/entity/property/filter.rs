use async_graphql::dynamic::ResolverContext;
use reactive_graph_graph::DataType;
use reactive_graph_graph::EntityType;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_service_api::ReactiveEntityManager;
use serde_json::Value;
use std::sync::Arc;

pub fn get_entity_instances_by_type_filter_by_properties(
    ctx: &ResolverContext,
    entity_type: &EntityType,
    entity_instance_manager: &Arc<dyn ReactiveEntityManager + Send + Sync>,
) -> Vec<ReactiveEntity> {
    let mut instances = entity_instance_manager.get_by_type(&entity_type.ty);
    for property in entity_type.properties.iter() {
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
    instances
}
