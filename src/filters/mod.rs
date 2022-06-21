mod eq;
pub use eq::Equal;
mod ge;
pub use ge::GreaterThan;
mod gt;
mod le;
mod lt;
mod ne;

pub type Filters<T, V> = Vec<Box<dyn Filter<T, V>>>;

pub trait Filter<T, Value> {
    fn compare(&self, value: T) -> bool;
    fn index(&self) -> usize;
}
