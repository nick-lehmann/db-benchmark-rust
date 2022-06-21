use std::ops::Index;

use crate::{filters::Filters, table::Table};

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

    fn query<const PROJECTION: usize>(
        &self,
        projection: [usize; PROJECTION],
        filters: Filters<T, T>,
    ) -> Vec<[T; PROJECTION]> {
        let mut result: Vec<[T; PROJECTION]> = Vec::new();

        for index in 0..=self.len() - 1 {
            let mut match_all = true;
            let row = &self[index];

            for filter in &filters {
                let cell = row.get(filter.index()).unwrap();
                if !filter.compare(cell.clone()) {
                    match_all = false;
                    break;
                }
            }

            if match_all {
                let mut new_row = [T::default(); PROJECTION];
                for i in 0..PROJECTION {
                    new_row[i] = row.get(projection[i]).unwrap().clone();
                }
                result.push(new_row);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        data::generate_data,
        filters::{Equal, Filters, GreaterThan},
    };

    #[test]
    fn test_basic_filters_only() {
        let data = generate_data::<u32, 3>(10);
        let filters: Filters<u32, u32> = vec![Box::new(Equal { index: 0, value: 5 })];
        let expected = vec![[5, 5, 5]];

        let row_table = RowTable::<u32, 3>::new(data);
        let result = row_table.filter(filters);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_complex_filters() {
        let data = generate_data::<u32, 3>(10);
        let filters: Filters<u32, u32> = vec![
            Box::new(Equal { index: 0, value: 5 }),
            Box::new(GreaterThan { index: 1, value: 3 }),
        ];
        let expected = vec![[5, 5, 5]];

        let row_table = RowTable::<u32, 3>::new(data);
        let result = row_table.filter(filters);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_projection_only() {
        let data = generate_data::<u32, 3>(3);
        let filters: Filters<u32, u32> = vec![];
        let projection = [0, 1];
        let expected = vec![[0, 0], [1, 1], [2, 2]];

        let row_table = RowTable::<u32, 3>::new(data);
        let result = row_table.query(projection, filters);

        assert_eq!(result, expected)
    }
}
