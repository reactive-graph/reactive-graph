use crate::cli::error::CommandError;
use crate::cli::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::components::type_id::queries::ComponentTypeIdVariables;
use reactive_graph_graph::ComponentTypeId;

/// The component type.
#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentTypeIdArgs {
    /// The component namespace.
    pub namespace: String,

    /// The component name.
    pub name: String,
}

impl ComponentTypeIdArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("Component {}__{} not found", &self.namespace, &self.name))
    }
}

impl From<ComponentTypeIdArgs> for ComponentTypeId {
    fn from(ty: ComponentTypeIdArgs) -> Self {
        ComponentTypeId::new_from_type(ty.namespace, ty.name)
    }
}

impl From<&ComponentTypeIdArgs> for ComponentTypeIdVariables {
    fn from(ty: &ComponentTypeIdArgs) -> Self {
        let ty: ComponentTypeId = ty.clone().into();
        ty.into()
    }
}

/// The component type in a component container.
#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentContainerTypeIdArgs {
    /// The component namespace.
    pub component_namespace: String,

    /// The component name.
    pub component_name: String,
}

impl From<ComponentContainerTypeIdArgs> for ComponentTypeId {
    fn from(ty: ComponentContainerTypeIdArgs) -> Self {
        ComponentTypeId::new_from_type(ty.component_namespace, ty.component_name)
    }
}