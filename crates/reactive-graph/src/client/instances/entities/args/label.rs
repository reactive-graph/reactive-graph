use crate::client::error::CommandError;
use crate::client::error::CommandError::NotFound;
use clap::Args;

/// CLI argument which identifies a reactive instance by its label.
#[derive(Args, Debug, Clone)]
pub(crate) struct LabelArgs {
    /// The label of the reactive instance.
    pub label: String,
}

impl LabelArgs {
    pub fn not_found(&self) -> CommandError {
        NotFound(format!("The instance with the label {} was not found", &self.label))
    }
}

impl From<LabelArgs> for String {
    fn from(label: LabelArgs) -> Self {
        label.label
    }
}
