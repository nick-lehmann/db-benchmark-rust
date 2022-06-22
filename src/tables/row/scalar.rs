use crate::{
    filters::ScalarFilters,
    tables::{ScalarQuery, Table},
};

use super::RowTable;

impl<Data: std::fmt::Debug + Copy + Default, const ATTRS: usize> ScalarQuery<Data>
    for RowTable<Data, ATTRS>
{
    fn query<const PROJECTION: usize>(
        &self,
        projection: [usize; PROJECTION],
        filters: ScalarFilters<Data, Data>,
    ) -> Vec<[Data; PROJECTION]> {
        let mut result: Vec<[Data; PROJECTION]> = Vec::new();

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
                let mut new_row = [Data::default(); PROJECTION];
                for i in 0..PROJECTION {
                    new_row[i] = row.get(projection[i]).unwrap().clone();
                }
                result.push(new_row);
            }
        }

        result
    }
}
