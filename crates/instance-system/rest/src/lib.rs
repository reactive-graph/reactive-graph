use actix_web::HttpResponse;
use schemars::Schema;

pub mod entities;
pub mod flows;
pub mod relations;

pub(crate) fn json_schema_response(schema: Schema) -> HttpResponse {
    HttpResponse::Ok().content_type("application/schema+json".to_string()).json(schema)
}
