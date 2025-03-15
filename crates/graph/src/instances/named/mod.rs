pub trait NamedInstanceContainer {
    /// Returns the name of the instance.
    fn name(&self) -> String;

    /// Returns the description of the instance.
    fn description(&self) -> String;
}

// pub struct NamedInstance {
//     pub name: String,
//     pub description: String,
// }
//
// impl NamedInstanceContainer for NamedInstance {
//     fn name(&self) -> String {
//         self.name.clone()
//     }
//
//     fn description(&self) -> String {
//         self.description.clone()
//     }
// }
//
// impl NamedInstanceContainer for Option<NamedInstance> {
//     fn name(&self) -> String {
//         match self {
//             None => Default::default(),
//             Some(named_instance) => named_instance.name.clone(),
//         }
//     }
//
//     fn description(&self) -> String {
//         match self {
//             None => Default::default(),
//             Some(named_instance) => named_instance.description.clone(),
//         }
//     }
// }
