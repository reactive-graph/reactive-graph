use serde_json::json;

use crate::model::component_model;
use crate::model::component_ty;
use crate::model::properties;
use crate::model_runtime::NAMESPACE_CORE;

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
