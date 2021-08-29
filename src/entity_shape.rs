// use crate::model::property_type::PropertyType;
// use indradb::Type;
// use serde::{Deserialize, Serialize};
// use std::str::FromStr;
// use serde_json::Value;
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct ShapeElement {
//
//     #[serde(default = "default_true")]
//     pub show: bool,
//
//     #[serde(default)]
//     pub font_size: i64,
//
//     #[serde(default = "empty_string")]
//     pub content: String,
//
// }
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct SocketShapeDefinition {
//
//     #[serde(default = "default_socket_size")]
//     pub width: i64,
//
//     #[serde(default = "default_socket_size")]
//     pub height: i64,
//
// }
// impl SocketShapeDefinition {}
//
// #[derive(Serialize, Deserialize, Clone, Debug)]
// pub struct EntityShape {
//     pub width: i64,
//
//     pub socket: SocketShapeDefinition,
//
//     pub title: ShapeElement,
//
//     pub symbol: ShapeElement,
//
//     pub id: ShapeElement,
//
//     #[serde(default = "Vec::new")]
//     pub components: Vec<String>,
//
//     #[serde(default = "Vec::new")]
//     pub behaviours: Vec<String>,
//
//     #[serde(default = "Vec::new")]
//     pub properties: Vec<PropertyType>,
//
//     #[serde(default)]
//     pub shape: Value,
// }
//
// impl EntityShape {
//     pub fn new<S: Into<String>>(
//         name: S,
//         components: Vec<String>,
//         behaviours: Vec<String>,
//         properties: Vec<PropertyType>
//     ) -> EntityShape {
//         let name = name.into();
//         let t = Type::from_str(name.as_str()).unwrap();
//         EntityShape {
//             name,
//             description: empty_string(),
//             components,
//             behaviours,
//             properties,
//             shape: Default::default(),
//             t,
//         }
//     }
//
//     // /// Returns true, if the entity type is a.
//     // pub fn is_a<S: Into<String>>(&self, component_name: S) -> bool {
//     //     self.components.contains(&component_name.into())
//     // }
//     //
//     // /// Returns true, if the entity type behaves as.
//     // pub fn behaves_as<S: Into<String>>(&self, behaviour_name: S) -> bool {
//     //     self.behaviours.contains(&behaviour_name.into())
//     // }
//     //
//     // /// Returns true, if the entity type contains an own property with the given name.
//     // /// Doesn't respect properties from potential components.
//     // pub fn has_own_property<S: Into<String>>(&self, property_name: S) -> bool {
//     //     let property_name = property_name.into().clone();
//     //     !self.properties.iter()
//     //         .filter(|&p| p.name == property_name).collect::<Vec<_>>().is_empty()
//     // }
// }
//
// #[cfg_attr(tarpaulin, ignore)]
// fn empty_string() -> String {
//     "".to_string()
// }
//
// #[cfg_attr(tarpaulin, ignore)]
// fn default_false() -> bool {
//     false
// }
//
// #[cfg_attr(tarpaulin, ignore)]
// fn default_true() -> bool {
//     true
// }
//
// #[cfg_attr(tarpaulin, ignore)]
// fn default_socket_size() -> i64 {
//     30
// }
