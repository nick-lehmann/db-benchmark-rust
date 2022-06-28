use super::ColumnTable;
use crate::{
    filters::{VectorFilter, VectorFilters},
    tables::VectorisedQuery,
};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

impl<const ATTRS: usize> VectorisedQuery<i32> for ColumnTable<i32, ATTRS> {
    unsafe fn filter(&self, filters: VectorFilters<__m512i, i32, __mmask16>) -> Vec<i32> {
        let chunk_size = 16;

        let rows = self.data[0].len();
        // Stores the indices of the rows which have passed all filters that were already applied.
        // Adding `chunk_size` elements in the end is a hack to make sure we do not access invalid memory
        // when gathering the indices.
        let mut indices = vec![0i32; rows + chunk_size].into_boxed_slice();
        // Number of indices for the rows that matched all filters already checked.
        // Since no filters have been applied yet, this is the number of rows.
        let mut indices_counter: usize = rows;

        let ones_register = _mm512_set1_epi32(1);

        let mut first_run = true;
        for (column_index, column) in self.data.iter().enumerate() {
            let filter_for_current_columns: Vec<&Box<dyn VectorFilter<__m512i, i32, __mmask16>>> =
                filters
                    .iter()
                    .filter(|filter| filter.index() == column_index)
                    .collect();

            if filter_for_current_columns.len() == 0 {
                break;
            }

            let mut new_indices_counter = 0;
            for index in (0..indices_counter - 1).step_by(chunk_size) {
                let remaining_elements = rows - index;
                let shift = match remaining_elements >= chunk_size {
                    true => 0,
                    false => chunk_size - remaining_elements,
                };
                let mut match_mask: u16 = 0b1111_1111_1111_1111 >> shift;

                let (indices_register, data_register) = match first_run {
                    true => {
                        let indices_register = create_indices_register32(index as i32);
                        let data_register = _mm512_loadu_si512(&column[0]);
                        (indices_register, data_register)
                    }
                    false => {
                        // This line is why we added `chunk_size` elements to the end of `indices`.
                        let indices_block =
                            &indices[new_indices_counter..new_indices_counter + chunk_size];
                        let indices_register = create_indices_register32_from_slice(indices_block);
                        let data_register = _mm512_i32gather_epi32::<4>(
                            indices_register,
                            &column[0] as *const _ as *const u8,
                        );
                        (indices_register, data_register)
                    }
                };

                debug!(
                    "Indices register: {:?}",
                    std::mem::transmute::<__m512i, [i32; 16]>(indices_register)
                );
                debug!(
                    "Data register: {:?}",
                    std::mem::transmute::<__m512i, [i32; 16]>(indices_register)
                );

                for filter in &filter_for_current_columns {
                    match_mask = filter.compare(data_register, match_mask);
                }
                println!("Match mask: {:b}", match_mask);

                _mm512_mask_compressstoreu_epi32(
                    &mut indices[new_indices_counter] as *mut i32 as *mut u8,
                    match_mask,
                    indices_register,
                );
                debug!("Current indices: {:?}", indices);

                let added = _mm512_mask_reduce_add_epi32(match_mask, ones_register) as usize;
                new_indices_counter += added;
                first_run = false;
            }
            indices_counter = new_indices_counter;
        }

        indices[..indices_counter].to_vec()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        data::generate_data,
        filters::{Equal, GreaterEqual, LessEqual, VectorFilters},
        tables::{ColumnTable, Table, VectorisedQuery},
    };

    #[cfg(target_arch = "x86")]
    use std::arch::x86::*;
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::*;

    #[test]
    fn test_basic_filters() {
        let chunk_size = 16;
        let lengths = [chunk_size / 2, chunk_size - 1, chunk_size];

        for length in lengths {
            let data = generate_data::<i32, 3>(length);
            let filters: VectorFilters<__m512i, i32, __mmask16> =
                vec![Box::new(LessEqual::<i32>::new(0, 1))];
            let expected = vec![0, 1];

            let row_table = ColumnTable::<i32, 3>::new(data);
            let result = unsafe { row_table.filter(filters) };

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn test_multiple_filters() {
        let chunk_size = 16;
        let lengths = [chunk_size / 2, chunk_size - 1, chunk_size];

        for length in lengths {
            let data = generate_data::<i32, 3>(length);
            let filters: VectorFilters<__m512i, i32, __mmask16> = vec![
                Box::new(LessEqual::<i32>::new(0, 4)),
                Box::new(GreaterEqual::<i32>::new(1, 1)),
                Box::new(Equal::<i32>::new(2, 2)),
            ];
            let expected = vec![2];

            let row_table = ColumnTable::<i32, 3>::new(data);
            let result = unsafe { row_table.filter(filters) };

            assert_eq!(result, expected);
        }
    }
}

unsafe fn create_indices_register32(index: i32) -> __m512i {
    _mm512_set_epi32(
        index + 15,
        index + 14,
        index + 13,
        index + 12,
        index + 11,
        index + 10,
        index + 9,
        index + 8,
        index + 7,
        index + 6,
        index + 5,
        index + 4,
        index + 3,
        index + 2,
        index + 1,
        index + 0,
    )
}

unsafe fn create_indices_register32_from_slice(slice: &[i32]) -> __m512i {
    _mm512_set_epi32(
        slice[15], slice[14], slice[13], slice[12], slice[11], slice[10], slice[9], slice[8],
        slice[7], slice[6], slice[5], slice[4], slice[3], slice[2], slice[1], slice[0],
    )
}
