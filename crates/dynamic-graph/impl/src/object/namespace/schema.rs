use crate::field::to_field_value;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::TypeRef;
use schemars::Schema;

pub const FIELD_JSON_SCHEMA_APPENDIX: &str = "JsonSchema";

pub fn json_schema_field<TY: Into<DynamicGraphTypeDefinition>>(ty: TY, schema: Schema) -> Field {
    Field::new(
        ty.into().field_name_with_suffix_and_appendix(FIELD_JSON_SCHEMA_APPENDIX),
        TypeRef::named_nn("JSON"),
        move |_ctx| {
            let schema = schema.clone();
            FieldFuture::new(async move { Ok(to_field_value(schema.as_value().clone())) })
        },
    )
}
