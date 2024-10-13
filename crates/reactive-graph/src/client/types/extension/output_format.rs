use crate::client::output_format::OutputFormatWrapper;

pub(crate) type ExtensionsOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::Extension,
    reactive_graph_table_model::types::extension::Extension,
    reactive_graph_table_model::types::extension::ExtensionsTableOptions,
>;
