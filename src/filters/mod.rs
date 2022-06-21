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

pub trait Filter<T, Value>: IndexContainer {
    fn compare(&self, value: T) -> bool;
}
