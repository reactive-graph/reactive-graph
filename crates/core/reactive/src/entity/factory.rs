use crate::model::ReactiveEntityInstance;
use crate::BehaviourFactory;

pub trait EntityBehaviourFactory: BehaviourFactory<ReactiveEntityInstance> + Send + Sync {}
