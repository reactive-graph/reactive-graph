use crate::client::output_format::OutputFormatWrapper;

pub(crate) type FlowTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::FlowType,
    reactive_graph_table_model::types::flows::FlowType,
    reactive_graph_table_model::types::flows::FlowTypesTableOptions,
>;
