use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::property_type::PropertyTypesTableOptions;

pub(crate) type PropertyTypesOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::PropertyType, table_model::types::property_type::PropertyType, PropertyTypesTableOptions>;
