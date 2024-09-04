use crate::cli::output_format::OutputFormatWrapper;

pub(crate) type EntityInstancesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::EntityInstance,
    reactive_graph_table_model::instances::entities::EntityInstance,
    reactive_graph_table_model::instances::entities::EntityInstancesTableOptions,
>;
