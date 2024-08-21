use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::instances::entities::EntityInstancesTableOptions;

pub(crate) type EntityInstancesOutputFormatWrapper =
    OutputFormatWrapper<reactive_graph_graph::EntityInstance, table_model::instances::entities::EntityInstance, EntityInstancesTableOptions>;
