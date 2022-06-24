use crate::{
    filters::ScalarFilters,
    tables::{ScalarQuery, Table},
};

use super::RowTable;

impl<Data: std::fmt::Debug + Copy + Default, const ATTRS: usize> ScalarQuery<Data>
    for RowTable<Data, ATTRS>
{
    fn filter(&self, filters: ScalarFilters<Data, Data>) -> Vec<i32> {
        let mut indices: Vec<i32> = vec![];

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
                indices.push(index as i32);
            }
        }

        indices
    }
}
