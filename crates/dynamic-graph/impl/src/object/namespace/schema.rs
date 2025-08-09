use crate::field::to_field_value;
use crate::object::types::DynamicGraphTypeDefinition;
use async_graphql::dynamic::Field;
use async_graphql::dynamic::FieldFuture;
use async_graphql::dynamic::TypeRef;
use convert_case::Case::Lower;
use convert_case::Casing;
use reactive_graph_graph::TypeDefinitionGetter;
use schemars::Schema;

pub const FIELD_JSON_SCHEMA_APPENDIX: &str = "JsonSchema";

pub fn json_schema_field<TY: Into<DynamicGraphTypeDefinition>>(ty: TY, schema: Schema) -> Field {
    let ty = ty.into();
    let type_definition = ty.type_definition();
    Field::new(
        ty.into().field_name_with_suffix_and_appendix(FIELD_JSON_SCHEMA_APPENDIX),
        TypeRef::named_nn("JSON"),
        move |_ctx| {
            let schema = schema.clone();
            FieldFuture::new(async move { Ok(to_field_value(schema.as_value().clone())) })
        }
    )
    .description(format!(
        "JSON schema of {} {}",
        type_definition.type_id_type.full_name().to_case(Lower),
        type_definition.namespaced_type
    ))
}
