//! Small calculator helpers.

mod arithmetic;
mod midpoint;
mod noise;
mod percent;
mod tax;

pub use arithmetic::{add, mul, safe_div, sub};
pub use midpoint::midpoint;
pub use percent::percent;
pub use tax::sales_tax_cents;
