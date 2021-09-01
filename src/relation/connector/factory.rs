// TODO: Is it necessary to refactor this factory ?

use std::str::FromStr;
use std::sync::Arc;

use indradb::{Type, Edge, EdgeProperties, EdgeKey};

use crate::model::{ReactiveRelationInstance, ReactiveRelationInstanceFactory, ReactiveEntityInstance};
use crate::relation::connector::ConnectorProperties;

pub struct ConnectorReactiveRelationInstanceFactory {}
impl ReactiveRelationInstanceFactory for ConnectorReactiveRelationInstanceFactory {
    fn new<S: Into<String>>(
        outbound: Arc<ReactiveEntityInstance>,
        type_name: S,
        inbound: Arc<ReactiveEntityInstance>
    ) -> Arc<ReactiveRelationInstance> {
        let key = EdgeKey::new(
            outbound.id,
            Type::from_str(type_name.into().as_str()).unwrap(),
            inbound.id
        );
        Arc::new(ReactiveRelationInstance::from(
            outbound.clone(),
            inbound.clone(),
            EdgeProperties::new(
                Edge::new_with_current_datetime(key),
                ConnectorProperties::properties()
            )
        ))
    }
}
