use crate::field::to_field_value;
use crate::field::to_type_ref;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use reactive_graph_graph::PropertyInstanceGetter;
use reactive_graph_graph::PropertyType;
use reactive_graph_reactive_model_impl::ReactiveEntity;

pub fn entity_property_field(property_type: &PropertyType) -> Field {
    let property_type_inner = property_type.clone();
    Field::new(&property_type.name, to_type_ref(&property_type.data_type), move |ctx| {
        let property_type = property_type_inner.clone();
        FieldFuture::new(async move {
            let entity_instance = ctx.parent_value.try_downcast_ref::<ReactiveEntity>()?;
            Ok(entity_instance.get(&property_type.name).and_then(to_field_value))
        })
    })
    .description(&property_type.description)
}
