use crate::cli::output_format::OutputFormatWrapper;

pub(crate) type EntityTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::EntityType,
    reactive_graph_table_model::types::entity_type::EntityType,
    reactive_graph_table_model::types::entity_type::EntityTypesTableOptions,
>;
