use springtime_di::Component;

use reactive_graph_graph::Components;
use reactive_graph_graph::EntityTypes;
use reactive_graph_graph::FlowTypes;
use reactive_graph_graph::RelationTypes;
use reactive_graph_type_system_api::TypeProvider;

#[derive(TypeProvider, Component)]
#[type_provider(tys = "Components", path = "../types/components")]
pub struct CommandComponentsProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "EntityTypes", path = "../types/entities")]
pub struct CommandEntityTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "RelationTypes", path = "../types/relations")]
pub struct CommandRelationTypesProvider {}

#[derive(TypeProvider, Component)]
#[type_provider(tys = "FlowTypes", path = "../types/flows")]
pub struct CommandFlowTypesProvider {}
