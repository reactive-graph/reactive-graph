use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::RelationInstance;

pub fn schema_relation_instances() -> Schema {
    schema_for!(RelationInstance)
}
