use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::FlowInstance;

pub fn schema_flow_instances() -> Schema {
    schema_for!(FlowInstance)
}
