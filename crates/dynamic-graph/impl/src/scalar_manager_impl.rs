use async_graphql::dynamic::Scalar;
use async_trait::async_trait;
use springtime_di::Component;
use springtime_di::component_alias;

use reactive_graph_dynamic_graph_api::ScalarManager;
use reactive_graph_lifecycle::Lifecycle;

#[derive(Component)]
pub struct ScalarManagerImpl {}

#[async_trait]
#[component_alias]
impl ScalarManager for ScalarManagerImpl {
    fn get_scalars(&self) -> Vec<Scalar> {
        vec![Scalar::new("JSON")]
    }
}

#[async_trait]
impl Lifecycle for ScalarManagerImpl {}
