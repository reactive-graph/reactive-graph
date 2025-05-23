use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::FlowType;

pub fn schema_flow_types() -> Schema {
    schema_for!(FlowType)
}
