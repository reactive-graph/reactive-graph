use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::EntityType;

pub fn schema_entity_types() -> Schema {
    schema_for!(EntityType)
}
