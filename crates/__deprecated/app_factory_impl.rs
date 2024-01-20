use crate::get_logger_middleware;
use actix_cors::Cors;
use actix_web::web;
use actix_web::App;
use actix_web_extras::middleware::Condition;
use async_trait::async_trait;
use inexor_rgf_config_api::ConfigManager;
use inexor_rgf_lifecycle::Lifecycle;
use inexor_rgf_plugin_api::ComponentManager;
use inexor_rgf_plugin_api::EntityTypeManager;
use inexor_rgf_plugin_api::FlowTypeManager;
use inexor_rgf_plugin_api::RelationTypeManager;
use inexor_rgf_runtime_web_api::AppFactory;
use springtime_di::component_alias;
use springtime_di::Component;
use std::sync::Arc;

#[derive(Component)]
pub struct AppFactoryImpl {
    config_manager: Arc<dyn ConfigManager + Send + Sync>,

    component_manager: Arc<dyn ComponentManager + Send + Sync>,

    entity_type_manager: Arc<dyn EntityTypeManager + Send + Sync>,

    relation_type_manager: Arc<dyn RelationTypeManager + Send + Sync>,

    flow_type_manager: Arc<dyn FlowTypeManager + Send + Sync>,
}

impl Lifecycle for AppFactoryImpl {}

// impl AppFactoryImpl {
//     fn type_system(&self, app: x) {
//         let component_manager = web::Data::new(self.component_manager.clone());
//         let entity_type_manager = web::Data::new(self.entity_type_manager.clone());
//         let relation_type_manager = web::Data::new(self.relation_type_manager.clone());
//         let flow_type_manager = web::Data::new(self.flow_type_manager.clone());
//         app.app_data(component_manager.clone())
//             .app_data(entity_type_manager.clone())
//             .app_data(relation_type_manager.clone())
//             .app_data(flow_type_manager.clone())
//     }
// }

#[async_trait]
#[component_alias]
impl AppFactory for AppFactoryImpl {
    fn create_app(&self) {
        // let graphql_server_config = self.config_manager.get_graphql_server_config();
        // let graphql_logging_config = graphql_server_config.logging.as_ref().cloned().unwrap_or_default();
        // let mut app = App::new()
        //     .wrap(Cors::permissive())
        //     .wrap(Condition::from_option(get_logger_middleware(&graphql_logging_config)));
        // let mut app = app.app_data();
        // app
        // let app = self.type_system(app);
        // app
        // let mut app = app
        //     .app_data(component_manager.clone())
        //     .app_data(entity_type_manager.clone())
        //     .app_data(relation_type_manager.clone())
        //     .app_data(flow_type_manager.clone());
        // app.app_data(schema_data.clone())
        //     .app_data(entity_instance_manager.clone())
        //     .app_data(relation_instance_manager.clone())
        //     .app_data(flow_instance_manager.clone())
        //     .app_data(web_resource_manager.clone())
        //     .app_data(dynamic_graph_schema_manager.clone())
        //     // GraphQL API
        //     .service(query_graphql)
        //     .service(
        //         web::resource("/graphql")
        //             .guard(guard::Get())
        //             .guard(guard::Header("upgrade", "websocket"))
        //             .to(subscription_websocket),
        //     )
        //     // Dynamic GraphQL API
        //     .service(query_dynamic_graph)
        //     // REST API
        //     .service(crate::rest::types::components::get_components)
        //     .service(crate::rest::types::components::get_component)
        //     .service(crate::rest::types::components::schema_components)
        //     .service(crate::rest::types::entities::get_entity_types)
        //     .service(crate::rest::types::entities::get_entity_type)
        //     .service(crate::rest::types::entities::schema_entity_types)
        //     .service(crate::rest::types::relations::get_relation_types)
        //     .service(crate::rest::types::relations::get_relation_type)
        //     .service(crate::rest::types::relations::schema_relation_types)
        //     .service(crate::rest::types::flows::get_flow_types)
        //     .service(crate::rest::types::flows::get_flow_type)
        //     .service(crate::rest::types::flows::schema_flow_types)
        //     .service(crate::rest::instances::entities::schema_entity_instances)
        //     .service(crate::rest::instances::relations::schema_relation_instances)
        //     .service(crate::rest::instances::flows::schema_flow_instances)
        //     // Web Resource API
        //     .service(web::resource("/{web_resource_context_path}/{path:.*}").route(web::get().to(handle_web_resource)))
        //     .service(web::resource("/{path:.*}").route(web::get().to(handle_root_web_resource)));
    }
}
