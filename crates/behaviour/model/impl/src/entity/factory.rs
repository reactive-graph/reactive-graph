use uuid::Uuid;

use inexor_rgf_behaviour_model_api::prelude::*;

use inexor_rgf_reactive_model_impl::ReactiveEntity;

pub trait EntityBehaviourFactory: BehaviourFactory<Uuid, ReactiveEntity> + Send + Sync {}

pub type EntityBehaviourFactoryCreator<T> = BehaviourFactoryCreator<Uuid, ReactiveEntity, T>;

pub type EntityBehaviourFactories = BehaviourFactories<Uuid, ReactiveEntity>;
