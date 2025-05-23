use schemars::Schema;
use schemars::schema_for;

use reactive_graph_graph::Component;

pub fn schema_components() -> Schema {
    schema_for!(Component)
}
