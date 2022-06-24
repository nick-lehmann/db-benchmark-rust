use crate::{
    filters::{ScalarFilter, ScalarFilters},
    tables::{ScalarQuery, Table},
};

use super::ColumnTable;

impl<Data: std::fmt::Debug + Copy + Default, const ATTRS: usize> ScalarQuery<Data>
    for ColumnTable<Data, ATTRS>
{
    fn filter(&self, filters: ScalarFilters<Data, Data>) -> Vec<i32> {
        let mut indices: Vec<i32> = (0i32..=self.len() as i32 - 1).collect();

        for (column_index, column) in self.data.iter().enumerate() {
            let filter_for_current_columns: Vec<&Box<dyn ScalarFilter<Data, Data>>> = filters
                .iter()
                .filter(|filter| filter.index() == column_index)
                .collect();

            if filter_for_current_columns.len() == 0 {
                break;
            }

            let mut new_indices: Vec<i32> = Vec::new();
            for index in &indices {
                let mut match_all = true;
                for filter in &filter_for_current_columns {
                    let cell = column.get(index.clone() as usize).unwrap();

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

        indices
    }
}
