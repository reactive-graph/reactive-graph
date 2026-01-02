use std::ops::Deref;

use serde_json::Value;
use serde_json::json;
use typed_builder::TypedBuilder;
use uuid::Uuid;

use crate::entity::Command;
use crate::entity::CommandArgs;
use crate::reactive_graph::command::command::COMMAND;
use crate::reactive_graph::command::command::CommandProperties;
use reactive_graph_graph::ComponentTypeIds;
use reactive_graph_graph::EntityTypeId;
use reactive_graph_graph::NamespacedTypeGetter;
use reactive_graph_graph::PropertyInstanceSetter;
use reactive_graph_graph::PropertyInstances;
use reactive_graph_model_core::reactive_graph::core::action::ACTION;
use reactive_graph_model_core::reactive_graph::core::action::ActionProperties;
use reactive_graph_model_core::reactive_graph::core::labeled::LABELED;
use reactive_graph_model_core::reactive_graph::core::labeled::LabeledProperties;
use reactive_graph_reactive_model_impl::ReactiveEntity;
use reactive_graph_reactive_model_impl::ReactiveProperties;

pub type CommandExecutor = dyn FnMut(&ReactiveEntity) -> Value + 'static + Send;

#[derive(TypedBuilder)]
#[builder(
    build_method(vis="pub", into=Command),
    builder_method(vis="pub"),
    builder_type(vis="pub", name=CommandDefinitionBuilder),
)]
pub struct CommandDefinition {
    #[builder(setter(into))]
    ty: EntityTypeId,
    #[builder(default=Uuid::new_v4(), setter(into))]
    id: Uuid,
    #[builder(default, setter(strip_option, into))]
    namespace: Option<String>,
    #[builder(default, setter(strip_option, into))]
    name: Option<String>,
    #[builder(default, setter(into))]
    description: String,
    #[builder(default, setter(into))]
    help: String,
    #[builder(default, setter(into))]
    arguments: CommandArgs,
    executor: Box<CommandExecutor>,
}

impl From<CommandDefinition> for Command {
    fn from(definition: CommandDefinition) -> Self {
        let handle_id = Uuid::new_v4().as_u128();

        let namespace = definition.namespace.unwrap_or_else(|| definition.ty.path().to_string());
        let name = definition.name.unwrap_or_else(|| definition.ty.type_name().to_string());

        let label = format!("/io/reactive-graph/commands/{namespace}/{name}");

        let components = ComponentTypeIds::new()
            .component(LABELED.deref())
            .component(ACTION.deref())
            .component(COMMAND.deref());

        let properties = PropertyInstances::new()
            .property(LabeledProperties::LABEL.as_ref(), json!(label))
            .property(ActionProperties::TRIGGER.as_ref(), json!(false))
            .property(CommandProperties::NAMESPACE.as_ref(), json!(namespace))
            .property(CommandProperties::COMMAND.as_ref(), json!(name))
            .property(CommandProperties::ARGS.as_ref(), definition.arguments.to_value())
            .property(CommandProperties::HELP.as_ref(), json!(definition.help))
            .property(CommandProperties::CMD_RESULT.as_ref(), json!(0));

        for arg in definition.arguments.to_vec() {
            if !properties.contains_key(&arg.name) {
                properties.insert(arg.name.clone(), json!(0));
            }
        }

        let reactive_entity = ReactiveEntity::builder()
            .ty(definition.ty)
            .id(definition.id)
            .description(definition.description)
            .components(components)
            .properties(ReactiveProperties::new_with_id_from_properties(definition.id, properties))
            .build();
        let reactive_entity_inner = reactive_entity.clone();
        let mut executor = definition.executor;
        if let Some(property_instance) = reactive_entity.properties.get(ActionProperties::TRIGGER.as_ref()) {
            property_instance.stream.read().unwrap().observe_with_handle(
                move |trigger| {
                    if trigger.as_bool().unwrap_or_default() {
                        reactive_entity_inner.set(CommandProperties::CMD_RESULT.as_ref(), executor(&reactive_entity_inner));
                    }
                },
                handle_id,
            );
        };
        Command::new_unchecked(reactive_entity)
    }
}

#[cfg(test)]
mod tests {
    use crate::Command;
    use crate::CommandArgs;
    use reactive_graph_graph::EntityTypeId;
    use reactive_graph_reactive_model_impl::ReactiveEntity;
    use serde_json::json;
    use std::str::FromStr;

    #[test]
    fn command_builder_test() {
        let args = CommandArgs::new();
        // TODO: fill args
        let executor = Box::new(move |_: &ReactiveEntity| json!("abc"));
        let command = Command::builder()
            .ty(EntityTypeId::from_str("reactive_graph::command::NumCommands").unwrap())
            .description("The number of commands")
            .help("Number of commands")
            .arguments(args)
            .executor(executor)
            .build();

        assert_eq!(command.ty(), EntityTypeId::from_str("reactive_graph::command::NumCommands").unwrap());
    }
}
