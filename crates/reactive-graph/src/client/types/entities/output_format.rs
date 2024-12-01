use crate::client::output_format::OutputFormatWrapper;

pub(crate) type EntityTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::EntityType,
    reactive_graph_table_model::types::entities::EntityType,
    reactive_graph_table_model::types::entities::EntityTypesTableOptions,
>;
