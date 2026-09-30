//! Tiny calculator library with intentional math edge-case bugs.
//!
//! Production entry points live in `percent` and `midpoint`.
//! Decoy modules (`tax`, `noise`, `billing_hints`) are correct or irrelevant.

mod arithmetic;
mod midpoint;
mod noise;
mod percent;
mod tax;

pub use arithmetic::{add, mul, safe_div, sub};
pub use midpoint::midpoint;
pub use percent::percent;
pub use tax::sales_tax_cents;
