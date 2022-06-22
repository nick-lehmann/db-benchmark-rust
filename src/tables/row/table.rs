use crate::tables::Table;
use std::ops::Index;

pub struct RowTable<T, const ATTRS: usize> {
    data: Vec<[T; ATTRS]>,
}

impl<T, const ATTRS: usize> Index<usize> for RowTable<T, ATTRS> {
    type Output = [T; ATTRS];

    fn index(&self, index: usize) -> &Self::Output {
        return &self.data[index];
    }
}

impl<T: std::fmt::Debug + Copy + Default, const ATTRS: usize> Table<T, ATTRS>
    for RowTable<T, ATTRS>
{
    fn new(data: Vec<[T; ATTRS]>) -> Self {
        RowTable { data }
    }

    fn len(&self) -> usize {
        self.data.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::generate_data,
        filters::{Equal, GreaterEqual, ScalarFilters},
        tables::ScalarQuery,
    };

    #[test]
    fn test_basic_filters_only() {
        let data = generate_data::<i32, 3>(10);
        let filters: ScalarFilters<i32, i32> = vec![Box::new(Equal::<i32>::new(0, 5))];
        let expected = vec![[5, 5, 5]];

        let row_table = RowTable::new(data);
        let result = row_table.query([0, 1, 2], filters);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_filters() {
        let data = generate_data::<i32, 3>(10);
        let filters: ScalarFilters<i32, i32> = vec![
            Box::new(Equal::<i32>::new(0, 5)),
            Box::new(GreaterEqual::<i32>::new(1, 3)),
        ];
        let expected = vec![[5, 5, 5]];

        let row_table = RowTable::new(data);
        let result = row_table.query([0, 1, 2], filters);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_projection_only() {
        let data = generate_data::<i32, 3>(3);
        let filters: ScalarFilters<i32, i32> = vec![];
        let projection = [0, 1];
        let expected = vec![[0, 0], [1, 1], [2, 2]];

        let row_table = RowTable::new(data);
        let result = row_table.query(projection, filters);

        assert_eq!(result, expected)
    }
}
