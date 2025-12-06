use crate::object_type_name::object_type_name;
use async_graphql::dynamic::FieldValue;
use log::info;
use reactive_graph_dynamic_graph_api::RootObjectType;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_reactive_model_api::ReactiveInstanceUnidentifiable;
use std::sync::Arc;

pub fn reactive_instance_component<'a, T: ReactiveInstanceUnidentifiable + 'static>(
    reactive_instance: T,
    ty: &ComponentTypeId,
    root_object_type: RootObjectType,
) -> FieldValue<'a> {
    let type_name = match root_object_type {
        RootObjectType::Interface => object_type_name(ty, root_object_type),
        RootObjectType::Query => object_type_name(reactive_instance.namespaced_type(), root_object_type),
        RootObjectType::Mutation => object_type_name(reactive_instance.namespaced_type(), root_object_type),
    };
    info!("{ty} {} {root_object_type} -> {type_name}", reactive_instance.namespace());
    // let ty = match root_object_type {
    //     RootObjectType::Interface => reactive_instance.namespaced_type(),
    //     RootObjectType::Query => reactive_instance.namespaced_type(),
    //     RootObjectType::Mutation => ty.namespaced_type(),
    // };
    // let ty1 = ty.clone();
    // let ty = reactive_instance.namespaced_type();
    // let ty2 = ty.clone();
    // let type_name = object_type_name(ty, root_object_type);
    // info!("{ty1} {ty2} {root_object_type} -> {type_name}");
    let boxed_reactive_instance = Arc::<Box<dyn ReactiveInstanceUnidentifiable>>::new(Box::new(reactive_instance));
    FieldValue::owned_any(boxed_reactive_instance).with_type(type_name)
}

// use reactive_graph_graph::NamespacedTypeGetter;
// pub fn reactive_instance_component_old<'a, T: PropertyInstanceGetter + NamespacedTypeGetter + Send + Sync + 'static>(
//     reactive_instance: T,
//     root_object_type: RootObjectType,
// ) -> FieldValue<'a> {
//     let ty = reactive_instance.namespaced_type();
//     let type_name = object_type_name(ty, root_object_type);
//     FieldValue::owned_any(reactive_instance).with_type(type_name)
// }
//
