#![allow(unused_variables, dead_code, unused_mut)]
#![feature(int_log, stdsimd, fn_traits, test)]
extern crate test;

mod data;
mod filters;
mod tables;

pub use data::generate_data;
pub use filters::{
    Equal, GreaterEqual, LessEqual, ScalarFilter, ScalarFilters, VectorFilter, VectorFilters,
};
pub use tables::{ColumnTable, RowTable, ScalarQuery, Table, VectorisedQuery};
