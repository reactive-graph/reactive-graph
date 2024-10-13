use crate::client::output_format::OutputFormatWrapper;

pub(crate) type ComponentsOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::Component,
    reactive_graph_table_model::types::component::Component,
    reactive_graph_table_model::types::component::ComponentsTableOptions,
>;

pub(crate) type ComponentTypeIdsOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::ComponentTypeId,
    reactive_graph_table_model::types::component::ComponentTypeId,
    reactive_graph_table_model::types::component::ComponentTypeIdsTableOptions,
>;
