use crate::cli::output_format::OutputFormatWrapper;

pub(crate) type RelationInstancesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::RelationInstance,
    reactive_graph_table_model::instances::relations::RelationInstance,
    reactive_graph_table_model::instances::relations::RelationInstancesTableOptions,
>;
