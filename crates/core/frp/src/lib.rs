// unsized_tuple_coercion requires nightly
#![feature(unsized_tuple_coercion)]
#![feature(unboxed_closures)]
#![feature(register_tool)]
#![register_tool(tarpaulin)]
// Added in rust nightly 1.72.0
// This lint warns when you use Arc with a type that does not implement Send or Sync.
// Wrapping a type in Arc doesn't add thread safety to the underlying data, so data races could occur when touching the underlying data.
// TODO: use mutexes or DashMap
#![allow(clippy::arc_with_non_send_sync)]

/// Fork of bidule
/// Adds handle_ids for subscribers
pub mod frp;
pub use frp::*;

#[cfg(test)]
#[tarpaulin::ignore]
mod tests;
