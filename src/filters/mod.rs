mod eq;
pub use eq::Equal;
mod ge;
pub use ge::GreaterEqual;
// mod gt;
mod le;
pub use le::LessEqual;
// mod lt;
// mod ne;

pub type Filters<T, V> = Vec<Box<dyn Filter<T, V>>>;

pub trait IndexContainer {
    fn index(&self) -> usize;
}

pub trait Filter<Value, Input>: IndexContainer {
    fn compare(&self, value: Input) -> bool;
}

pub trait MaskedFilter<Input, Value, Mask> {
    fn compare(&self, value: Input, mask: Mask) -> Mask;
}
