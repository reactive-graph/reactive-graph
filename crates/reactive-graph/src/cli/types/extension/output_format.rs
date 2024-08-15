use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::extension::ExtensionsTableOptions;

pub(crate) type ExtensionsOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::Extension, table_model::types::extension::Extension, ExtensionsTableOptions>;
