use std::fmt::Display;
use std::sync::Arc;

use crate::ComponentContainer;
use crate::NamespacedTypeGetter;
use crate::PropertyInstanceSetter;
use crate::ReactiveBehaviourContainer;
use crate::ReactivePropertyContainer;

/// A reactive instance is a container for properties, components and behaviours.
/// Furthermore the reactive instance has a namespaced type.
pub trait ReactiveInstance:
    ReactivePropertyContainer + ComponentContainer + ReactiveBehaviourContainer + PropertyInstanceSetter + NamespacedTypeGetter + Display
{
}

pub trait ReactiveInstanceGetter<T> {
    /// Returns the reactive instance.
    fn get_reactive_instance(&self) -> &Arc<T>;
}
