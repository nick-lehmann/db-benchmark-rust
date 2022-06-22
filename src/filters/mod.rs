mod eq;
pub use eq::Equal;
mod ge;
pub use ge::GreaterEqual;
// mod gt;
mod le;
pub use le::LessEqual;
// mod lt;
// mod ne;

pub trait IndexContainer {
    fn index(&self) -> usize;
}

pub type ScalarFilters<Value, Input> = Vec<Box<dyn ScalarFilter<Value, Input>>>;
pub trait ScalarFilter<Value, Input>: IndexContainer {
    fn compare(&self, value: Input) -> bool;
}

pub type VectorFilters<Value, Input, Mask> = Vec<Box<dyn VectorFilter<Value, Input, Mask>>>;
pub trait VectorFilter<Input, Value, Mask> {
    fn compare(&self, value: Input, mask: Mask) -> Mask;
}
