use crate::{
    filters::{ScalarFilter, ScalarFilters},
    tables::{ScalarQuery, Table},
};

use super::ColumnTable;

impl<Data: std::fmt::Debug + Copy + Default, const ATTRS: usize> ScalarQuery<Data>
    for ColumnTable<Data, ATTRS>
{
    fn query<const PROJECTION: usize>(
        &self,
        projection: [usize; PROJECTION],
        filters: ScalarFilters<Data, Data>,
    ) -> Vec<[Data; PROJECTION]> {
        let mut indices: Vec<usize> = (0..=self.len() - 1).collect();

        for (column_index, column) in self.data.iter().enumerate() {
            let filter_for_current_columns: Vec<&Box<dyn ScalarFilter<Data, Data>>> = filters
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

        let mut result: Vec<[Data; PROJECTION]> = Vec::new();
        for index in indices {
            let mut row = [Data::default(); PROJECTION];
            for column in projection {
                row[column] = self.data[column].get(index).unwrap().clone();
            }
            result.push(row);
        }

        result
    }
}
