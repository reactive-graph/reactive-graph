#![feature(unsized_tuple_coercion)]

/// Fork of bidule
/// Adds handle_ids for subscribers
pub mod bidule;
pub use bidule::*;

#[cfg(test)]
#[cfg_attr(tarpaulin, ignore)]
mod tests;
