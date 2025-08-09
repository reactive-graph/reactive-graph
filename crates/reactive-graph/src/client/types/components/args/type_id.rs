use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use clap::Args;
use reactive_graph_client::types::common::variables::type_id::variables::TypeIdVariables;
use reactive_graph_graph::ComponentTypeId;
use reactive_graph_graph::NAMESPACE_SEPARATOR;
use std::fmt::Display;
use std::fmt::Formatter;

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
        NotFound(format!("Component {}{}{} not found", &self.namespace, NAMESPACE_SEPARATOR, &self.name))
    }
}

impl From<ComponentTypeIdArgs> for ComponentTypeId {
    fn from(ty: ComponentTypeIdArgs) -> Self {
        ComponentTypeId::new_from_type(ty.namespace, ty.name)
    }
}

impl From<&ComponentTypeIdArgs> for TypeIdVariables {
    fn from(ty: &ComponentTypeIdArgs) -> Self {
        let ty: ComponentTypeId = ty.clone().into();
        ty.into()
    }
}

impl Display for ComponentTypeIdArgs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", &self.namespace, NAMESPACE_SEPARATOR, &self.name)
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

/// The component as option.
#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentTypeIdOptions {
    /// The component namespace.
    #[clap(long)]
    pub namespace: Option<String>,

    /// The component name.
    #[clap(short, long)]
    pub name: Option<String>,
}

impl From<ComponentTypeIdOptions> for Option<ComponentTypeId> {
    fn from(ty: ComponentTypeIdOptions) -> Self {
        if ty.namespace.is_none() && ty.name.is_none() {
            return None;
        }
        Some(ComponentTypeId::new_from_type(ty.namespace.unwrap_or_default(), ty.name.unwrap_or_default()))
    }
}
