use crate::client::output_format::OutputFormatWrapper;

pub(crate) type FlowInstancesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::FlowInstance,
    reactive_graph_table_model::instances::flows::FlowInstance,
    reactive_graph_table_model::instances::flows::FlowInstancesTableOptions,
>;
