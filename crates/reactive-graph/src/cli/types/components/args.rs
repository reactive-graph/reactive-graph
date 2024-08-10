use crate::cli::types::components::commands::ComponentsCommands;
use crate::cli::types::extension::args::ExtensionDefinitionArgs;
use crate::cli::types::extension::args::ExtensionTypeIdArgs;
use crate::cli::types::property_type::args::PropertyTypeDefinitionArgs;
use clap::Args;
use reactive_graph_client::schema_graphql::scalar::Json;
use reactive_graph_client::schema_graphql::types::property_type::PropertyTypeDefinition;
use reactive_graph_client::types::components::add_extension::queries::AddExtensionVariables;
use reactive_graph_client::types::components::add_property::queries::AddPropertyVariables;
use reactive_graph_client::types::components::create_component::queries::CreateComponentVariables;
use reactive_graph_client::types::components::remove_extension::queries::RemoveExtensionVariables;
use reactive_graph_client::types::components::remove_property::queries::RemovePropertyVariables;
use reactive_graph_client::types::components::type_id::queries::ComponentTypeIdVariables;
use reactive_graph_client::types::components::update_description::queries::UpdateDescriptionVariables;
use reactive_graph_client::ExtensionDefinition;
use reactive_graph_graph::ComponentTypeId;

#[derive(Args, Debug, Clone)]
#[clap(subcommand_required = true)]
pub(crate) struct ComponentsArgs {
    #[command(subcommand)]
    pub(crate) commands: Option<ComponentsCommands>,
}

#[derive(Args, Debug, Clone)]
pub(crate) struct CreateComponentArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The component description.
    pub description: Option<String>,
}

impl From<&CreateComponentArgs> for CreateComponentVariables {
    fn from(args: &CreateComponentArgs) -> Self {
        CreateComponentVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
            properties: None,
            extensions: None,
        }
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddPropertyArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The property.
    #[clap(flatten)]
    pub property_type: PropertyTypeDefinitionArgs,
}

impl From<&ComponentAddPropertyArgs> for AddPropertyVariables {
    fn from(args: &ComponentAddPropertyArgs) -> Self {
        AddPropertyVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            property: PropertyTypeDefinition {
                name: args.property_type.property_name.clone(),
                description: args.property_type.description.clone().unwrap_or_default(),
                data_type: args.property_type.data_type.into(),
                socket_type: args.property_type.socket_type.into(),
                mutability: args.property_type.mutability.into(),
                extensions: Vec::new(),
            },
        }
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentRemovePropertyArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The name of the property.
    pub property_name: String,
}

impl From<&ComponentRemovePropertyArgs> for RemovePropertyVariables {
    fn from(args: &ComponentRemovePropertyArgs) -> Self {
        RemovePropertyVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            property_name: args.property_name.clone(),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentAddExtensionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The extension.
    #[clap(flatten)]
    pub extension: ExtensionDefinitionArgs,
}

impl From<&ComponentAddExtensionArgs> for AddExtensionVariables {
    fn from(args: &ComponentAddExtensionArgs) -> Self {
        let extension: Json = args.extension.extension.clone().into();
        let ty: reactive_graph_graph::ExtensionTypeId = args.extension.ty.clone().into();
        let ty: reactive_graph_client::ExtensionTypeId = ty.into();
        AddExtensionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            extension: ExtensionDefinition {
                type_: ty,
                description: args.extension.description.clone(),
                extension,
            },
        }
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentRemoveExtensionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The component type.
    #[clap(flatten)]
    pub extension_ty: ExtensionTypeIdArgs,
}

impl From<&ComponentRemoveExtensionArgs> for RemoveExtensionVariables {
    fn from(args: &ComponentRemoveExtensionArgs) -> Self {
        RemoveExtensionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            extension_namespace: args.extension_ty.extension_namespace.clone(),
            extension_name: args.extension_ty.extension_name.clone(),
        }
    }
}

#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentUpdateDescriptionArgs {
    /// The component type.
    #[clap(flatten)]
    pub ty: ComponentTypeIdArgs,

    /// The description to update.
    pub description: String,
}

impl From<&ComponentUpdateDescriptionArgs> for UpdateDescriptionVariables {
    fn from(args: &ComponentUpdateDescriptionArgs) -> Self {
        UpdateDescriptionVariables {
            namespace: args.ty.namespace.clone(),
            name: args.ty.name.clone(),
            description: args.description.clone(),
        }
    }
}

/// The component type.
#[derive(Args, Debug, Clone)]
pub(crate) struct ComponentTypeIdArgs {
    /// The component namespace.
    pub namespace: String,

    /// The component name.
    pub name: String,
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
