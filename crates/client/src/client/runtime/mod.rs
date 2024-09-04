use std::sync::Arc;

use crate::client::plugin::api::Plugins;
use crate::client::runtime::instance::api::Instance;
use crate::client::runtime::remotes::api::Remotes;
use crate::client::runtime::shutdown::api::Shutdown;
use crate::client::InexorRgfClient;
use command::api::Command;

pub mod command;
pub mod instance;
pub mod remotes;
pub mod shutdown;

pub struct Runtime {
    client: Arc<InexorRgfClient>,
}

impl Runtime {
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

    pub fn shutdown(&self) -> Shutdown {
        Shutdown::new(self.client.clone())
    }
}
