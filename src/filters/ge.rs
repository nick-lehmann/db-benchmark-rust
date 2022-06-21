use super::{Filter, IndexContainer};

pub struct GreaterEqual<Value> {
    pub index: usize,
    pub value: Value,
}

impl<Value> IndexContainer for GreaterEqual<Value> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<T, Value> Filter<T, Value> for GreaterEqual<Value>
where
    T: PartialOrd<Value>,
{
    fn compare(&self, value: T) -> bool {
        value >= self.value
    }
}
