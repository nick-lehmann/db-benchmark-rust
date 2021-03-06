use std::ops::Index;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

mod column;
pub use column::ColumnTable;
mod row;
pub use row::RowTable;

// mod pax_table;
// pub use pax_table::PaxTable;

use crate::filters::{ScalarFilters, VectorFilters};

pub trait Table<T: std::fmt::Debug + Copy, const ATTRS: usize>:
    Index<usize, Output = [T; ATTRS]>
{
    fn new(data: Vec<[T; ATTRS]>) -> Self;

    /// Returns the number of rows in the table.
    fn len(&self) -> usize;

    fn project<const COLUMNS: usize>(
        &self,
        projection: [usize; COLUMNS],
        indices: &[usize],
    ) -> Vec<[T; COLUMNS]>;

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

pub trait ScalarQuery<Data> {
    fn filter(&self, filters: &ScalarFilters<Data, Data>) -> Vec<i32>;
}

pub trait VectorisedQuery<Data> {
    unsafe fn filter(&self, filters: &VectorFilters<__m512i, Data, __mmask16>) -> Vec<i32>;
}
