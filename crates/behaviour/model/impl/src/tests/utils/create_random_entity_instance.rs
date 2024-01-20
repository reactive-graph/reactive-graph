use serde_json::json;
use uuid::Uuid;

use crate::tests::utils::r_string;
use inexor_rgf_graph::EntityInstance;
use inexor_rgf_graph::EntityTypeId;
use inexor_rgf_graph::PropertyInstances;
use inexor_rgf_reactive_model_impl::ReactiveEntity;

// TODO: replace with default_test()
pub fn create_random_entity_instance<S: Into<String>>(property_name: S) -> ReactiveEntity {
    let properties = PropertyInstances::new().property(property_name, json!(r_string()));
    let ty = EntityTypeId::new_from_type(r_string(), r_string());
    let entity_instance = EntityInstance::builder()
        .ty(ty)
        .id(Uuid::new_v4())
        .description(r_string())
        .properties(properties)
        .build();
    ReactiveEntity::from(entity_instance)
}
