// use std::fmt::Display;
//
// use inexor_rgf_reactive_api::prelude::*;
//
// use crate::model::NamespacedTypeGetter;
// use crate::model::PropertyInstanceSetter;
// use crate::ReactivePropertyContainer;
//
// /// A reactive instance is a container for properties and components.
// /// Furthermore the reactive instance has a namespaced type.
// pub trait ReactiveInstance<ID>: ReactivePropertyContainer + ComponentContainer + PropertyInstanceSetter + NamespacedTypeGetter + Display {
//     /// Returns the id of the reactive instance.
//     fn id(&self) -> ID;
// }
//
// pub trait ReactiveInstanceGetter<T> {
//     /// Returns the reactive instance.
//     fn get_reactive_instance(&self) -> &T;
// }
