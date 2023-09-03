use dashmap::DashSet;

pub type Namespaces = DashSet<String>;

pub use namespaced_type_container::*;

pub mod namespaced_type_container;
