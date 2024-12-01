use crate::client::output_format::OutputFormatWrapper;

pub(crate) type PropertyTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::PropertyType,
    reactive_graph_table_model::types::properties::PropertyType,
    reactive_graph_table_model::types::properties::PropertyTypesTableOptions,
>;
