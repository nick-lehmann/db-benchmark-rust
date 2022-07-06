mod eq;
pub use eq::Equal;
mod ge;
pub use ge::GreaterEqual;
// mod gt;
mod le;
pub use le::LessEqual;
// mod lt;
// mod ne;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use crate::VectorisedQuery;

pub trait IndexContainer {
    fn index(&self) -> usize;
}

pub type ScalarFilters<Value, Input> = Vec<Box<dyn ScalarFilter<Value, Input>>>;
pub trait ScalarFilter<Value, Input>: IndexContainer {
    fn compare(&self, value: Input) -> bool;
}

pub type VectorFilters<Input, Value, Mask> = Vec<Box<dyn VectorFilter<Input, Value, Mask>>>;
pub trait VectorFilter<Input, Value, Mask>: IndexContainer {
    fn compare(&self, value: Input, mask: Mask) -> Mask;
}

pub trait Filter<Value>:
    ScalarFilter<Value, Value> + VectorFilter<__m512i, Value, __mmask16>
{
}

impl<T> Filter<i32> for T where T: ScalarFilter<i32, i32> + VectorFilter<__m512i, i32, __mmask16> {}
