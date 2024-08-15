use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::entity_type::EntityTypesTableOptions;

pub(crate) type EntityTypesOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::EntityType, table_model::types::entity_type::EntityType, EntityTypesTableOptions>;
