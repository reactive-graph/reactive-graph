use crate::client::output_format::OutputFormatWrapper;

pub(crate) type PropertyTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::PropertyType,
    reactive_graph_table_model::types::property_type::PropertyType,
    reactive_graph_table_model::types::property_type::PropertyTypesTableOptions,
>;
