use std::sync::Arc;

use crate::client::system::command::api::Command;
use crate::client::system::instance::api::Instance;
use crate::client::system::plugin::api::Plugins;
use crate::client::system::remotes::api::Remotes;
use crate::client::InexorRgfClient;

pub mod command;
pub mod instance;
pub mod plugin;
pub mod remotes;

pub struct System {
    client: Arc<InexorRgfClient>,
}

impl System {
    pub fn new(client: Arc<InexorRgfClient>) -> Self {
        Self { client }
    }

    pub fn command(&self) -> Command {
        Command::new(self.client.clone())
    }

    pub fn plugins(&self) -> Plugins {
        Plugins::new(self.client.clone())
    }

    pub fn remotes(&self) -> Remotes {
        Remotes::new(self.client.clone())
    }

    pub fn instance(&self) -> Instance {
        Instance::new(self.client.clone())
    }
}
