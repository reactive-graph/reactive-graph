use crate::client::output_format::OutputFormatWrapper;

pub(crate) type PropertyInstancesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_table_model::instances::properties::PropertyInstance,
    reactive_graph_table_model::instances::properties::PropertyInstance,
    reactive_graph_table_model::instances::properties::PropertyInstancesTableOptions,
>;
