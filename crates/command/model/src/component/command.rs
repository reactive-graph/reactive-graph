use serde_json::json;

use inexor_rgf_graph::component_model;
use inexor_rgf_graph::component_ty;
use inexor_rgf_graph::properties;
use inexor_rgf_graph::DataType;
use inexor_rgf_graph::Mutability::Immutable;
use inexor_rgf_graph::PropertyType;
use inexor_rgf_graph::PropertyTypeDefinition;
use inexor_rgf_graph::PropertyTypes;
use inexor_rgf_graph::SocketType;
use inexor_rgf_runtime_model::ActionProperties::TRIGGER;
use inexor_rgf_runtime_model::LabeledProperties::LABEL;
use inexor_rgf_runtime_model::NAMESPACE_CORE;

properties!(
    CommandProperties,
    (COMMAND_NAMESPACE, "namespace", ""),
    (COMMAND_NAME, "command", ""),
    (COMMAND_ARGS, "args", json!([])),
    (COMMAND_HELP, "help", ""),
    (COMMAND_RESULT, "cmd_result", "")
);

component_ty!(COMPONENT_COMMAND, NAMESPACE_CORE, COMPONENT_NAME_COMMAND, "command");

component_model!(
    CommandComponent,
    get scope string,
    get command string,
    get help string,
);

lazy_static::lazy_static! {
    pub static ref COMMAND_PROPERTIES: PropertyTypes = PropertyTypes::new()
        .property(
            PropertyType::builder()
                .name(LABEL.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build()
        )
        .property(PropertyType::input(TRIGGER.property_name(), DataType::Bool))
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_NAMESPACE.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build()
        )
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_NAME.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build()
        )
        .property(PropertyType::input(CommandProperties::COMMAND_ARGS.property_name(), DataType::Array))
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_HELP.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build()
        )
        .property(PropertyType::output(CommandProperties::COMMAND_RESULT.property_name(), DataType::Any));
}
