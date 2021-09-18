// use crate::model::{EntityInstance, Flow};
// use uuid::Uuid;
// use std::collections::HashMap;
// use crate::tests::utils::r_string;
// use indradb::{VertexProperties, Vertex, Type, NamedProperty};
// use std::str::FromStr;
// use serde_json::json;
// use crate::model::{MutablePropertyInstanceSetter, PropertyInstanceGetter};
// use crate::builder::EntityInstanceBuilder;
//
// #[test]
// fn flow_test() {
//     let wrapper_entity_instance = EntityInstanceBuilder::new("and_3".to_string())
//         .property(ASDFASDFbit_1".to_string(), json!(false))
//         .property("bit_2".to_string(), json!(false))
//         .property("bit_3".to_string(), json!(false))
//         .property("result_1".to_string(), json!(false))
//         .get();
//
//     let and1_entity_instance = EntityInstanceBuilder::new("and".to_string())
//         .property("bit_1".to_string(), json!(false))
//         .property("bit_2".to_string(), json!(false))
//         .property("result_1".to_string(), json!(false))
//         .get();
//
//     let and2_entity_instance = EntityInstanceBuilder::new("and".to_string())
//         .property("bit_1".to_string(), json!(false))
//         .property("bit_2".to_string(), json!(false))
//         .property("result_1".to_string(), json!(false))
//         .get();
//
//     let mut flow = Flow::from(wrapper_entity_instance);
//     flow.entities.push(and1_entity_instance);
//     flow.entities.push(and2_entity_instance);
//
//
//     let uuid = Uuid::new_v4();
//     let type_name = r_string();
//     let description = r_string();
//     let properties = HashMap::new();
//     let entity_instance = EntityInstance {
//         type_name: type_name.clone(),
//         id: uuid.clone(),
//         description: description.to_string(),
//         properties: properties.clone(),
//     };
//     assert_eq!(type_name.clone(), entity_instance.type_name.clone());
//     assert_eq!(uuid.clone(), entity_instance.id.clone());
//     assert_eq!(description.clone(), entity_instance.description.clone());
//     assert_eq!(properties.clone(), entity_instance.properties.clone());
// }
