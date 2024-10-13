use crate::client::output_format::OutputFormatWrapper;

pub type RelationTypesOutputFormatWrapper = OutputFormatWrapper<
    reactive_graph_graph::RelationType,
    reactive_graph_table_model::types::relation_type::RelationType,
    reactive_graph_table_model::types::relation_type::RelationTypesTableOptions,
>;
