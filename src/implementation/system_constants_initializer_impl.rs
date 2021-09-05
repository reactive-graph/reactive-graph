// TODO: Move to plugin inexor-rgf-plugin-system

use async_trait::async_trait;
use log::debug;
use waiter_di::*;

use crate::api::{EntityTypeManager, Lifecycle, ReactiveFlowManager, SystemConstantsInitializer};
use crate::behaviour::EntityBehaviour;
use crate::builder::{
    DefaultConnectorBuilder, EntityInstanceBuilder, EntityTypeBuilder, FlowBuilder,
};
use crate::model::{Flow, ReactiveFlow};
use inexor_ecs_model::{DataType, PropertyType};
use inexor_ecs_type_system_inout::InputGateProperties;
use inexor_ecs_type_system_inout::LogDebug;
use inexor_ecs_type_system_inout::LogInfo;
use inexor_ecs_type_system_inout::StdIn;
use inexor_ecs_type_system_inout::{KeyDown, OutputGateProperties};
use inexor_ecs_type_system_value::{ValueGate, ValueGateProperties};
use serde_json::json;
use std::convert::TryFrom;

const SYSTEM_CONSTANTS_TYPE_NAME: &'static str = "system_constants";
const SYSTEM_CONSTANTS_NUM_CPUS: &'static str = "num_cpus";

#[wrapper]
pub struct SystemConstantsFlow(std::sync::RwLock<Option<std::sync::Arc<ReactiveFlow>>>);

#[provides]
fn create_external_type_dependency() -> SystemConstantsFlow {
    SystemConstantsFlow(std::sync::RwLock::new(None))
}

#[component]
pub struct SystemConstantsInitializerImpl {
    entity_type_manager: Wrc<dyn EntityTypeManager>,
    reactive_flow_manager: Wrc<dyn ReactiveFlowManager>,

    system_constants: SystemConstantsFlow,
}

#[async_trait]
#[provides]
impl SystemConstantsInitializer for SystemConstantsInitializerImpl {}

impl Lifecycle for SystemConstantsInitializerImpl {
    fn init(&self) {
        debug!("SystemConstantsInitializer::activate");

        let e_flow_type = EntityTypeBuilder::new(SYSTEM_CONSTANTS_TYPE_NAME)
            .property_from(PropertyType::output(
                SYSTEM_CONSTANTS_NUM_CPUS,
                DataType::Number,
            ))
            .register(self.entity_type_manager.clone());

        let e_flow = EntityInstanceBuilder::from(e_flow_type.clone())
            .property(SYSTEM_CONSTANTS_NUM_CPUS, json!(0))
            .get();

        let e_num_cpus = EntityInstanceBuilder::new(ValueGate::DEFAULT_TYPE_NAME)
            .property(
                ValueGateProperties::VALUE.to_string(),
                json!(num_cpus::get()),
            )
            .get();
        let e_log_info = EntityInstanceBuilder::new(LogInfo::TYPE_NAME.to_string())
            .property(OutputGateProperties::OUTPUT.to_string(), json!(0))
            .get();
        let r_num_cpus = DefaultConnectorBuilder::new()
            .outbound(e_num_cpus.clone(), ValueGateProperties::VALUE.to_string())
            .inbound(e_flow.clone(), SYSTEM_CONSTANTS_NUM_CPUS)
            .get();
        let r_log_num_cpus = DefaultConnectorBuilder::new()
            .outbound(e_num_cpus.clone(), ValueGateProperties::VALUE.to_string())
            .inbound(e_log_info.clone(), OutputGateProperties::OUTPUT.to_string())
            .get();

        let e_stdin = EntityInstanceBuilder::new(StdIn::TYPE_NAME.to_string())
            // .property(InputGateProperties::CONFIG.to_string(), json!(""))
            .property(InputGateProperties::INPUT.to_string(), json!(""))
            .get();
        let e_log_stdin = EntityInstanceBuilder::new(LogDebug::TYPE_NAME.to_string())
            .property(OutputGateProperties::OUTPUT.to_string(), json!(0))
            .get();
        let r_log_stdin = DefaultConnectorBuilder::new()
            .outbound(e_stdin.clone(), InputGateProperties::INPUT.to_string())
            .inbound(
                e_log_stdin.clone(),
                OutputGateProperties::OUTPUT.to_string(),
            )
            .get();
        let e_key_down = EntityInstanceBuilder::new(KeyDown::TYPE_NAME.to_string())
            .property(InputGateProperties::CONFIG.to_string(), json!("Escape"))
            .property(InputGateProperties::INPUT.to_string(), json!(""))
            .get();
        let r_log_key_down = DefaultConnectorBuilder::new()
            .outbound(e_key_down.clone(), InputGateProperties::INPUT.to_string())
            .inbound(
                e_log_stdin.clone(),
                OutputGateProperties::OUTPUT.to_string(),
            )
            .get();

        let system_constants = FlowBuilder::new(e_flow.clone())
            .entity(e_num_cpus.clone())
            .entity(e_log_info.clone())
            .relation(r_num_cpus.clone())
            .relation(r_log_num_cpus.clone())
            .entity(e_stdin.clone())
            .entity(e_log_stdin.clone())
            .relation(r_log_stdin.clone())
            .entity(e_key_down.clone())
            .relation(r_log_key_down.clone())
            .get();
        let reactive_flow = self.reactive_flow_manager.create(system_constants);
        if reactive_flow.is_ok() {
            let reactive_flow = reactive_flow.unwrap();
            debug!("Before tick");
            reactive_flow.tick();
            debug!("After tick");
            let flow = Flow::try_from(reactive_flow.clone());
            if flow.is_ok() {
                debug!("{}", serde_json::to_string_pretty(&flow.unwrap()).unwrap());
            }
            let mut writer = self.system_constants.0.write().unwrap();
            *writer = Some(reactive_flow);
        }
    }

    fn shutdown(&self) {
        debug!("SystemConstantsInitializer::deactivate");
        let mut writer = self.system_constants.0.write().unwrap();
        *writer = None;
    }
}
