use crate::cli::output_format::OutputFormatWrapper;
use crate::table_model;
use crate::table_model::types::property_type::PropertyTypesTableOptions;
use serde_json::Value;

pub(crate) type PropertyInstancesOutputFormatWrapper =
    OutputFormatWrapper<(String, Value), table_model::instances::properties::PropertyInstance, PropertyTypesTableOptions>;
