use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use log::{debug, error};
use waiter_di::{component, provides, wrapper};

use crate::api::ComponentManager;
use crate::api::Lifecycle;
use crate::model::PropertyType;
use crate::plugins::ComponentProvider;

#[wrapper]
pub struct Components(RwLock<std::vec::Vec<crate::model::Component>>);

pub struct ComponentManagerImpl {
    components: Components,
}

#[component]
impl ComponentManagerImpl {
    #[provides]
    fn new() -> Self {
        Self {
            components: Components(RwLock::new(std::vec::Vec::new())),
        }
    }
}

#[async_trait]
#[provides]
impl ComponentManager for ComponentManagerImpl {
    fn register(&self, component: crate::model::Component) {
        if !self.has(component.name.clone()) {
            debug!("Registered component {}", component.name);
            self.components.0.write().unwrap().push(component);
        }
    }

    // Returns a copy
    fn get_components(&self) -> Vec<crate::model::Component> {
        self.components.0.read().unwrap().to_vec()
    }

    fn has(&self, name: String) -> bool {
        self.get(name).is_some()
    }

    fn get(&self, name: String) -> Option<crate::model::Component> {
        self.components
            .0
            .read()
            .unwrap()
            .to_vec()
            .into_iter()
            .find(|component| component.name == name)
    }

    fn create(&self, name: String, properties: Vec<PropertyType>) {
        self.register(crate::model::Component::new(
            name.clone(),
            properties.to_vec(),
        ));
    }

    fn delete(&self, name: String) {
        self.components
            .0
            .write()
            .unwrap()
            .retain(|component| component.name != name);
    }

    fn import(&self, path: String) {
        let file = File::open(path);
        if file.is_ok() {
            let file = file.unwrap();
            let reader = BufReader::new(file);
            let component = serde_json::from_reader(reader);
            if component.is_ok() {
                self.register(component.unwrap());
            }
        }
    }

    fn export(&self, name: String, path: String) {
        let o_component = self.get(name.clone());
        if o_component.is_some() {
            let r_file = File::create(path.clone());
            match r_file {
                Ok(file) => {
                    let result = serde_json::to_writer_pretty(&file, &o_component.unwrap());
                    if result.is_err() {
                        error!(
                            "Failed to export component {} to {}: {}",
                            name,
                            path,
                            result.err().unwrap()
                        );
                    }
                }
                Err(error) => {
                    error!(
                        "Failed to export component {} to {}: {}",
                        name,
                        path,
                        error.to_string()
                    );
                }
            }
        }
    }

    fn add_provider(&self, component_provider: Arc<dyn ComponentProvider>) {
        for component in component_provider.get_components() {
            debug!("Registering component: {}", component.name);
            self.register(component);
        }
    }
}

impl Lifecycle for ComponentManagerImpl {
    fn init(&self) {
        // let mut components = Vec::new();
        // for component in components {
        //     debug!("Registering component: {}", component.name);
        //     self.register(component);
        // }
    }

    fn shutdown(&self) {
        // TODO: remove?
        self.components.0.write().unwrap().clear();
    }
}
