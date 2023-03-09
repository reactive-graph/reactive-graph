// use serde::Deserialize;
// use serde::Serialize;
// use serde_json::from_value;
// use serde_json::Error;
// use serde_json::Value;
//
// use crate::model::TypeDefinition;
//
// #[derive(Serialize, Deserialize)]
// pub(crate) struct TypeComponentIdentifier {
//     pub type_definition: TypeDefinition,
//     pub component: String,
// }
//
// impl TryFrom<Value> for TypeComponentIdentifier {
//     type Error = Error;
//
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         let type_component_identifier: Result<Self, Error> = from_value(value);
//         type_component_identifier
//     }
// }
//
// #[derive(Serialize, Deserialize)]
// pub(crate) struct TypePropertyIdentifier {
//     pub name: String,
//     pub property_name: String,
// }
//
// impl TryFrom<Value> for TypePropertyIdentifier {
//     type Error = Error;
//
//     fn try_from(value: Value) -> Result<Self, Self::Error> {
//         let type_property_identifier: Result<Self, Error> = from_value(value);
//         type_property_identifier
//     }
// }
