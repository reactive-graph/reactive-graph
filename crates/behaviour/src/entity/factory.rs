use uuid::Uuid;

use inexor_rgf_behaviour_api::prelude::*;

use crate::reactive::ReactiveEntity;

pub trait EntityBehaviourFactory: BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync {}

pub type EntityBehaviourFactoryCreator<T> = BehaviourFactoryCreator<Uuid, ReactiveEntity, T>;

pub type EntityBehaviourFactories = BehaviourFactories<Uuid, ReactiveEntity>;
