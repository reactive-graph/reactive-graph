use reactive_graph_graph::RelationType;
use schemars::Schema;
use schemars::schema_for;

pub fn schema_relation_types() -> Schema {
    schema_for!(RelationType)
}
