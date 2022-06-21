use super::Filter;

pub struct Equal<Value> {
    pub index: usize,
    pub value: Value,
}

impl<T, Value> Filter<T, Value> for Equal<Value>
where
    T: PartialEq<Value>,
{
    fn compare(&self, value: T) -> bool {
        value == self.value
    }

    fn index(&self) -> usize {
        self.index
    }
}
