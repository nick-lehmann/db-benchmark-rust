use crate::tables::Table;
use aligned::{Aligned, A64};
use std::ops::Index;

pub struct ColumnTable<T, const ATTRS: usize> {
    pub data: Aligned<A64, [Vec<T>; ATTRS]>,
}

impl<T, const ATTRS: usize> Index<usize> for ColumnTable<T, ATTRS> {
    type Output = [T; ATTRS];

    fn index(&self, _: usize) -> &Self::Output {
        // return &self.data[index];
        todo!();
    }
}

impl<Data: std::fmt::Debug + Copy + Default, const ATTRS: usize> Table<Data, ATTRS>
    for ColumnTable<Data, ATTRS>
{
    fn new(input: Vec<[Data; ATTRS]>) -> Self {
        let mut column_data: [Vec<Data>; ATTRS] = [(); ATTRS].map(|_| Vec::<Data>::new());

        for row in input {
            for i in 0..ATTRS {
                column_data[i].push(row[i]);
            }
        }

        ColumnTable {
            data: Aligned(column_data),
        }
    }

    fn len(&self) -> usize {
        self.data[0].len()
    }

    fn project<const COLUMNS: usize>(
        &self,
        projection: [usize; COLUMNS],
        indices: &[usize],
    ) -> Vec<[Data; COLUMNS]> {
        let mut result: Vec<[Data; COLUMNS]> = Vec::new();

        for index in indices {
            let mut row = [Data::default(); COLUMNS];
            for column in projection {
                row[column] = self.data[column]
                    .get(index.clone() as usize)
                    .unwrap()
                    .clone();
            }
            result.push(row);
        }

        result
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::{
//         data::generate_data,
//         filters::{Equal, Filters, GreaterEqual},
//     };

//     #[test]
//     fn test_basic_filters_only() {
//         let data = generate_data::<u32, 3>(10);
//         let filters: Filters<u32, u32> = vec![Box::new(Equal { index: 0, value: 5 })];
//         let expected = vec![[5, 5, 5]];

//         let row_table = ColumnTable::<u32, 3>::new(data);
//         let result = row_table.filter(filters);

//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_complex_filters() {
//         let data = generate_data::<u32, 3>(10);
//         let filters: Filters<u32, u32> = vec![
//             Box::new(Equal { index: 0, value: 5 }),
//             Box::new(GreaterEqual { index: 1, value: 3 }),
//         ];
//         let expected = vec![[5, 5, 5]];

//         let row_table = ColumnTable::<u32, 3>::new(data);
//         let result = row_table.filter(filters);

//         assert_eq!(result, expected);
//     }

//     #[test]
//     fn test_projection_only() {
//         let data = generate_data::<u32, 3>(3);
//         let filters: Filters<u32, u32> = vec![];
//         let projection = [0, 1];
//         let expected = vec![[0, 0], [1, 1], [2, 2]];

//         let row_table = ColumnTable::<u32, 3>::new(data);
//         let result = row_table.query(projection, filters);

//         assert_eq!(result, expected)
//     }
// }
