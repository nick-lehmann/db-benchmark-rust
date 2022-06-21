use super::Filter;

pub struct GreaterThan<Value> {
    pub index: usize,
    pub value: Value,
}

impl<T, Value> Filter<T, Value> for GreaterThan<Value>
where
    T: PartialOrd<Value>,
{
    fn compare(&self, value: T) -> bool {
        value > self.value
    }

    fn index(&self) -> usize {
        self.index
    }
}
