use super::{Filter, IndexContainer};

pub struct Equal<Value> {
    pub index: usize,
    pub value: Value,
}

impl<Value> IndexContainer for Equal<Value> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<T, Value> Filter<T, Value> for Equal<Value>
where
    T: PartialEq<Value>,
{
    fn compare(&self, value: T) -> bool {
        value == self.value
    }
}
