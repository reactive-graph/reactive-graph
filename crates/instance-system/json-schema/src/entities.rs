use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::EntityInstance;

pub fn schema_entity_instances() -> Schema {
    schema_for!(EntityInstance)
}
