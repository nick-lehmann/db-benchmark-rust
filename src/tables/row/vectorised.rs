use super::RowTable;
use crate::{filters::VectorFilters, tables::VectorisedQuery};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl<const ATTRS: usize> VectorisedQuery<i32> for RowTable<i32, ATTRS> {
    /// Currently only works if the numbers of rows is a multiple of 16.
    unsafe fn filter(&self, filters: &VectorFilters<__m512i, i32, __mmask16>) -> Vec<i32> {
        let mut indices: Vec<i32> = vec![];

        let mut columns = filters.iter().map(|f| f.index()).collect::<Vec<usize>>();
        columns.sort_unstable();
        columns.dedup();

        let tuple_width = ATTRS * ::core::mem::size_of::<i32>();
        let index_register = get_index_register(tuple_width as u32);
        // let index_register_contents = std::mem::transmute::<__m512i, [i32; 16]>(index_register);

        let ones_mask = _mm512_int2mask(0xffff);

        for row in (0..self.data.len()).step_by(16) {
            let mut mask = ones_mask;

            for column_index in &columns {
                let ptr =
                    (&self.data[row] as *const i32).offset(*column_index as isize) as *const u8;

                // Load cells for the next `chunk_size` rows.
                let data_register = _mm512_i32gather_epi32(index_register, ptr, 1);

                let data_register_content =
                    std::mem::transmute::<__m512i, [i32; 16]>(data_register);

                mask = filters
                    .iter()
                    .filter(|f| f.index() == *column_index)
                    .fold(mask, |mask, filter| filter.compare(data_register, mask));
            }

            for i in 0..16 {
                if mask & (0b1000_0000_0000_0000 >> i) != 0 {
                    indices.push(row as i32 + i);
                }
            }
        }

        indices
    }
}

unsafe fn get_index_register(tuple_width: u32) -> __m512i {
    _mm512_set_epi32(
        0 * tuple_width as i32,
        1 * tuple_width as i32,
        2 * tuple_width as i32,
        3 * tuple_width as i32,
        4 * tuple_width as i32,
        5 * tuple_width as i32,
        6 * tuple_width as i32,
        7 * tuple_width as i32,
        8 * tuple_width as i32,
        9 * tuple_width as i32,
        10 * tuple_width as i32,
        11 * tuple_width as i32,
        12 * tuple_width as i32,
        13 * tuple_width as i32,
        14 * tuple_width as i32,
        15 * tuple_width as i32,
    )
}

#[cfg(test)]
mod tests {
    use crate::{
        data::generate_data,
        filters::{Equal, GreaterEqual, LessEqual, VectorFilters},
        generate_random_data,
        tables::{RowTable, Table, VectorisedQuery},
    };

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[test]
    fn test_random_data() {
        let chunk_size = 16;
        let lengths = [chunk_size, 10_000];
        // let lengths = [10_000];

        for length in lengths {
            let data = generate_random_data::<3>(&length);
            // println!("{:?}", data);
            let filters: VectorFilters<__m512i, i32, __mmask16> = vec![
                Box::new(Equal::<i32>::new(0, 1126014292)),
                Box::new(LessEqual::<i32>::new(0, 2000000000)),
            ];
            let expected = vec![3];

            let table = RowTable::<i32, 3>::new(data);
            let result = unsafe { table.filter(&filters) };

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_empty_return_after_first_run() {
        let data = generate_random_data::<3>(&10_000);
        let table = RowTable::<i32, 3>::new(data);

        let filters: VectorFilters<__m512i, i32, __mmask16> =
            vec![Box::new(Equal::<i32>::new(0, i32::MAX))];
        let result = unsafe { table.filter(&filters) };

        assert_eq!(result.len(), 0);
    }

    /// Test a correct abort if the second filter discards all indices found by the first.
    #[test]
    fn test_empty_return_after_second_run() {
        let data = generate_random_data::<3>(&10_000);
        let table = RowTable::<i32, 3>::new(data);

        let filters: VectorFilters<__m512i, i32, __mmask16> = vec![
            Box::new(GreaterEqual::<i32>::new(0, 0)),
            Box::new(Equal::<i32>::new(0, i32::MAX)),
        ];
        let result = unsafe { table.filter(&filters) };

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_basic_filters() {
        let chunk_size = 16;
        let lengths = [chunk_size, chunk_size * 2, chunk_size * 100];

        for length in lengths {
            let data = generate_data::<i32, 3>(length);
            let filters: VectorFilters<__m512i, i32, __mmask16> =
                vec![Box::new(LessEqual::<i32>::new(0, 1))];
            let expected = vec![0, 1];

            let table = RowTable::<i32, 3>::new(data);
            let result = unsafe { table.filter(&filters) };

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_multiple_filters() {
        let chunk_size = 16;
        let lengths = [chunk_size];

        for length in lengths {
            let data = generate_data::<i32, 3>(length);
            let filters: VectorFilters<__m512i, i32, __mmask16> = vec![
                Box::new(LessEqual::<i32>::new(0, 4)),
                Box::new(GreaterEqual::<i32>::new(1, 1)),
                Box::new(Equal::<i32>::new(2, 2)),
            ];
            let expected = vec![2];

            let table = RowTable::<i32, 3>::new(data);
            let result = unsafe { table.filter(&filters) };

            assert_eq!(result, expected);
        }
    }
}
