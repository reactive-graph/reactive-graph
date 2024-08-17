use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::component::ComponentsTableOptions;

pub(crate) type ComponentsOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::Component, table_model::types::component::Component, ComponentsTableOptions>;