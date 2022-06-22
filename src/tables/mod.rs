mod column_table;
use std::ops::Index;

pub use column_table::ColumnTable;

mod row_table;
pub use row_table::RowTable;

// mod pax_table;
// pub use pax_table::PaxTable;

use crate::filters::Filters;

pub trait Table<T: std::fmt::Debug + Copy, const ATTRS: usize>:
    Index<usize, Output = [T; ATTRS]>
{
    fn new(data: Vec<[T; ATTRS]>) -> Self;

    fn filter(&self, filters: Filters<T, T>) -> Vec<[T; ATTRS]> {
        let projection: [usize; ATTRS] = (0..ATTRS).collect::<Vec<_>>().try_into().unwrap();
        self.query(projection, filters)
    }

    fn query<const PROJECTION: usize>(
        &self,
        projection: [usize; PROJECTION],
        filters: Filters<T, T>,
    ) -> Vec<[T; PROJECTION]>;

    /// Returns the number of rows in the table.
    fn len(&self) -> usize;

    fn print(&self) {
        let len = self.len();
        let index_width = (len.log10() + 1) as usize;

        for index in 0..=len - 1 {
            print!("{:index_width$}:", index);
            for attr in 0..ATTRS {
                print!(" {:8?}", self[index][attr]);
            }
            println!("");
        }
    }
}
