use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::relation_type::RelationTypesTableOptions;

pub(crate) type RelationTypesOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::RelationType, table_model::types::relation_type::RelationType, RelationTypesTableOptions>;
