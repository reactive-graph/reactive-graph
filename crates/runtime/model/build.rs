use reactive_graph_command_types::CommandComponentsProviderAssets;
use reactive_graph_generator_documentation::GenerateDocumentations;
use reactive_graph_graph::NamespacedTypeComponentPropertiesContainer;
use reactive_graph_runtime_types::RuntimeComponentsProviderAssets;
use reactive_graph_runtime_types::RuntimeEntityTypesProviderAssets;
use reactive_graph_runtime_types::RuntimeFlowTypesProviderAssets;
use reactive_graph_runtime_types::RuntimeRelationTypesProviderAssets;

fn main() -> anyhow::Result<()> {
    println!("cargo:rerun-if-changed=../types");

    let components = RuntimeComponentsProviderAssets::get_components();
    components.write_documentations()?;

    components.push_all(CommandComponentsProviderAssets::get_components());

    let entity_types = RuntimeEntityTypesProviderAssets::get_entity_types();
    entity_types.merge_component_properties(components.clone())?;
    entity_types.write_documentations()?;

    let relation_types = RuntimeRelationTypesProviderAssets::get_relation_types();
    relation_types.merge_component_properties(components)?;
    relation_types.write_documentations()?;

    let flow_types = RuntimeFlowTypesProviderAssets::get_flow_types();
    flow_types.write_documentations()?;

    Ok(())
}
