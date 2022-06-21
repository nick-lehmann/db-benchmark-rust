use super::{Filter, IndexContainer};

pub struct LessEqual<Value> {
    pub index: usize,
    pub value: Value,
}

impl<Value> IndexContainer for LessEqual<Value> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<T, Value> Filter<T, Value> for LessEqual<Value>
where
    T: PartialOrd<Value>,
{
    fn compare(&self, value: T) -> bool {
        value <= self.value
    }
}
