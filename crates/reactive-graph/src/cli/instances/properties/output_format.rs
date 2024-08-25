use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::instances::properties::PropertyInstancesTableOptions;

pub(crate) type PropertyInstancesOutputFormatWrapper = OutputFormatWrapper<
    table_model::instances::properties::PropertyInstance,
    table_model::instances::properties::PropertyInstance,
    PropertyInstancesTableOptions,
>;
