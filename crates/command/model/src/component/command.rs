use serde_json::json;
use std::sync::LazyLock;

use reactive_graph_graph::DataType;
use reactive_graph_graph::Mutability::Immutable;
use reactive_graph_graph::PropertyType;
use reactive_graph_graph::PropertyTypeDefinition;
use reactive_graph_graph::PropertyTypes;
use reactive_graph_graph::SocketType;
use reactive_graph_graph::component_model;
use reactive_graph_graph::component_ty;
use reactive_graph_graph::properties;
use reactive_graph_runtime_model::ActionProperties::TRIGGER;
use reactive_graph_runtime_model::LabeledProperties::LABEL;
use reactive_graph_runtime_model::NAMESPACE_CORE;

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

pub static COMMAND_PROPERTIES: LazyLock<PropertyTypes> = LazyLock::new(|| {
    PropertyTypes::new()
        .property(
            PropertyType::builder()
                .name(LABEL.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build(),
        )
        .property(PropertyType::input(TRIGGER.property_name(), DataType::Bool))
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_NAMESPACE.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build(),
        )
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_NAME.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build(),
        )
        .property(PropertyType::input(CommandProperties::COMMAND_ARGS.property_name(), DataType::Array))
        .property(
            PropertyType::builder()
                .name(CommandProperties::COMMAND_HELP.property_name())
                .data_type(DataType::String)
                .mutability(Immutable)
                .socket_type(SocketType::None)
                .build(),
        )
        .property(PropertyType::output(CommandProperties::COMMAND_RESULT.property_name(), DataType::Any))
});
