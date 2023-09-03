use uuid::Uuid;
use crate::reactive::ReactiveEntity;
use crate::BehaviourFactory;

pub trait EntityBehaviourFactory: BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync {}
