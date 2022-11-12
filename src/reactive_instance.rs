use std::fmt::Display;

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
