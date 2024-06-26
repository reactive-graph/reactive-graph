// TODO: flow_instance_import_export_manager_impl.rs

// use std::fs::File;
// use std::io::BufReader;
//
// use async_trait::async_trait;
// use log::error;
//
// use crate::api::{EntityInstanceImportExportManager, ReactiveEntityManager};
// use crate::api::FlowInstanceCreationError;
// use crate::api::FlowInstanceImportError;
// use crate::api::FlowInstanceManager;
// use crate::api::RelationInstanceManager;
// use crate::di::*;
// use reactive_graph_graph::FlowInstance;
//
// #[component]
// pub struct FlowInstanceManagerImpl {
//     reactive_entity_manager: Arc<dyn ReactiveEntityManager>,
//
//     relation_instance_manager: Arc<dyn RelationInstanceManager>,
// }
//
// #[async_trait]
// #[provides]
// impl FlowInstanceManager for FlowInstanceManagerImpl {
//     fn create(&self, flow_instance: FlowInstance) -> Result<FlowInstance, FlowInstanceCreationError> {
//         for entity_instance in flow_instance.entity_instances.iter() {
//             if !self.reactive_entity_manager.has(entity_instance.id) {
//                 let _id = self.reactive_entity_manager.create_from_instance(entity_instance.clone())?;
//             }
//         }
//         for relation_instance in flow_instance.relation_instances.iter() {
//             if !self.relation_instance_manager.has(&relation_instance.get_key()) {
//                 let _id = self.relation_instance_manager.create_from_instance(relation_instance.clone())?;
//             }
//         }
//         Ok(flow_instance)
//     }
//
//     fn commit(&self, flow_instance: FlowInstance) {
//         for entity_instance in flow_instance.entity_instances {
//             if self.entity_instance_manager.has(entity_instance.id) {
//                 // The entity instance has been updated
//                 self.entity_instance_manager.commit(entity_instance.clone());
//             } else {
//                 // The entity instance has been added
//                 let _result = self.entity_instance_manager.create_from_instance(entity_instance.clone());
//             }
//             // TODO: what happens with removed entity instances?
//         }
//         for relation_instance in flow_instance.relation_instances {
//             if self.relation_instance_manager.has(&relation_instance.get_key()) {
//                 // The relation instance has been updated
//                 self.relation_instance_manager.commit(relation_instance.clone());
//             } else {
//                 // The relation instance has been added
//                 let _result = self.relation_instance_manager.create_from_instance(relation_instance.clone());
//             }
//             // TODO: what happens with removed relation instances?
//         }
//     }
//
//     fn delete(&self, flow_instance: FlowInstance) {
//         // Reverse order: first relations then entities
//         for relation_instance in flow_instance.relation_instances {
//             self.relation_instance_manager.delete(&relation_instance.get_key());
//         }
//         for entity_instance in flow_instance.entity_instances {
//             self.entity_instance_manager.delete(entity_instance.id);
//         }
//     }
//
//     fn import(&self, path: &str) -> Result<FlowInstance, FlowInstanceImportError> {
//         let file = File::open(path)?;
//         let reader = BufReader::new(file);
//         let flow_instance: FlowInstance = serde_json::from_reader(reader)?;
//         self.create(flow_instance).map_err(|e| e.into())
//     }
//
//     fn export(&self, flow_instance: FlowInstance, path: &str) {
//         let r_file = File::create(path);
//         match r_file {
//             Ok(file) => {
//                 let result = serde_json::to_writer_pretty(&file, &flow_instance);
//                 if result.is_err() {
//                     error!("Failed to export flow instance {} to {}: {}", flow_instance.id, path, result.err().unwrap());
//                 }
//             }
//             Err(error) => {
//                 error!("Failed to export flow instance {} to {}: {}", flow_instance.id, path, error.to_string());
//             }
//         }
//     }
// }
