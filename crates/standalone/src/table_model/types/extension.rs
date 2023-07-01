use std::fmt;
use std::fmt::Formatter;
use std::ops::Deref;

use serde_json::Value;
use tabled::Table;
use tabled::Tabled;

use crate::model::NamespacedTypeGetter;

#[derive(Tabled)]
pub struct ExtensionTypeId {
    pub name: String,
    pub namespace: String,
}

impl From<crate::model::ExtensionTypeId> for ExtensionTypeId {
    fn from(ty: crate::model::ExtensionTypeId) -> Self {
        ExtensionTypeId {
            name: ty.type_name(),
            namespace: ty.namespace(),
        }
    }
}

pub struct ExtensionDefinition {
    pub type_: ExtensionTypeId,
    pub description: String,
    pub extension: Value,
}

impl From<crate::model::Extension> for ExtensionDefinition {
    fn from(extension: crate::model::Extension) -> Self {
        ExtensionDefinition {
            type_: extension.ty.into(),
            description: extension.description,
            extension: extension.extension,
        }
    }
}

pub fn display_extensions(extensions: &Vec<Extension>) -> String {
    Table::new(extensions).to_string()
}

pub struct ExtensionDefinitions(pub Vec<ExtensionDefinition>);

impl From<ExtensionDefinitions> for Vec<ExtensionDefinition> {
    fn from(extensions: ExtensionDefinitions) -> Self {
        extensions.0.into_iter().collect()
    }
}

impl From<Vec<crate::model::Extension>> for ExtensionDefinitions {
    fn from(extensions: Vec<crate::model::Extension>) -> Self {
        ExtensionDefinitions(extensions.into_iter().map(|extension| extension.into()).collect())
    }
}

#[derive(Clone, Debug, Tabled)]
pub struct Extension {
    /// The namespace of the extension.
    pub namespace: String,

    /// The name of the extension.
    pub name: String,

    /// Textual description of the extension.
    pub description: String,

    /// The extension as JSON representation.
    pub extension: Value,
}

impl From<Extension> for crate::model::Extension {
    fn from(extension: Extension) -> Self {
        let ty = crate::model::ExtensionTypeId::new_from_type(extension.namespace, extension.name);
        crate::model::Extension {
            ty,
            description: extension.description,
            extension: extension.extension,
        }
    }
}

impl From<crate::model::Extension> for Extension {
    fn from(extension: crate::model::Extension) -> Self {
        Extension {
            namespace: extension.namespace(),
            name: extension.type_name(),
            description: extension.description,
            extension: extension.extension,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Extensions(pub Vec<Extension>);

impl Deref for Extensions {
    type Target = Vec<Extension>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Extensions> for Vec<crate::model::Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.0.into_iter().map(|extension| extension.into()).collect()
    }
}

impl From<Extensions> for Vec<Extension> {
    fn from(extensions: Extensions) -> Self {
        extensions.0
    }
}

impl From<Vec<crate::model::Extension>> for Extensions {
    fn from(extensions: Vec<crate::model::Extension>) -> Self {
        Extensions(extensions.into_iter().map(Into::into).collect())
    }
}

impl fmt::Display for Extensions {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", Table::new(self.0.iter().cloned()))
    }
}
