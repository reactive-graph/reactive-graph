use crate::field::json::to_field_value;
use crate::type_ref::TYPE_REF_JSON;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use serde::Serialize;

pub const FIELD_NAME_EXPORT: &str = "export";

pub fn create_export_field<T: Serialize + 'static>() -> Field {
    Field::new(FIELD_NAME_EXPORT, TYPE_REF_JSON.clone(), move |ctx| {
        FieldFuture::new(async move {
            let reactive_instance = ctx.parent_value.try_downcast_ref::<T>()?;
            let value = serde_json::to_value(reactive_instance)?;
            Ok(to_field_value(value))
        })
    })
    .description("Export as JSON")
}
