use std::sync::Arc;

use crate::client::system::command::Command;
use crate::client::system::plugin::api::Plugins;
use crate::client::InexorRgfClient;

pub mod command;
pub mod plugin;

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
}
