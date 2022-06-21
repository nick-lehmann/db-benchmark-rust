use std::ops::Index;

use crate::{
    filters::{Filter, Filters},
    table::Table,
};

pub struct ColumnTable<T, const ATTRS: usize> {
    data: [Vec<T>; ATTRS],
}

impl<T, const ATTRS: usize> Index<usize> for ColumnTable<T, ATTRS> {
    type Output = [T; ATTRS];

    fn index(&self, _: usize) -> &Self::Output {
        // return &self.data[index];
        todo!();
    }
}

impl<T: std::fmt::Debug + Copy + Default, const ATTRS: usize> Table<T, ATTRS>
    for ColumnTable<T, ATTRS>
{
    fn new(input: Vec<[T; ATTRS]>) -> Self {
        let mut column_data: [Vec<T>; ATTRS] = [(); ATTRS].map(|_| Vec::<T>::new());

        for row in input {
            for i in 0..ATTRS {
                column_data[i].push(row[i]);
            }
        }

        ColumnTable { data: column_data }
    }

    fn len(&self) -> usize {
        self.data[0].len()
    }

    fn query<const PROJECTION: usize>(
        &self,
        projection: [usize; PROJECTION],
        filters: Filters<T, T>,
    ) -> Vec<[T; PROJECTION]> {
        let mut indices: Vec<usize> = (0..=self.len() - 1).collect();

        for (column_index, column) in self.data.iter().enumerate() {
            let filter_for_current_columns: Vec<&Box<dyn Filter<T, T>>> = filters
                .iter()
                .filter(|filter| filter.index() == column_index)
                .collect();

            if filter_for_current_columns.len() == 0 {
                break;
            }

            let mut new_indices: Vec<usize> = Vec::new();
            for index in &indices {
                let mut match_all = true;
                for filter in &filter_for_current_columns {
                    let cell = column.get(index.clone()).unwrap();

                    if !filter.compare(cell.clone()) {
                        match_all = false;
                        break;
                    }
                }

                if match_all {
                    new_indices.push(index.clone());
                }
            }

            indices = new_indices;
        }

        let mut result: Vec<[T; PROJECTION]> = Vec::new();
        for index in indices {
            let mut row = [T::default(); PROJECTION];
            for column in projection {
                row[column] = self.data[column].get(index).unwrap().clone();
            }
            result.push(row);
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

        let row_table = ColumnTable::<u32, 3>::new(data);
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

        let row_table = ColumnTable::<u32, 3>::new(data);
        let result = row_table.filter(filters);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_projection_only() {
        let data = generate_data::<u32, 3>(3);
        let filters: Filters<u32, u32> = vec![];
        let projection = [0, 1];
        let expected = vec![[0, 0], [1, 1], [2, 2]];

        let row_table = ColumnTable::<u32, 3>::new(data);
        let result = row_table.query(projection, filters);

        assert_eq!(result, expected)
    }
}
