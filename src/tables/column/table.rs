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

        ColumnTable {
            data: Aligned(column_data),
        }
    }

    fn len(&self) -> usize {
        self.data[0].len()
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
