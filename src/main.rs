#![feature(int_log)]
pub mod column;
pub mod data;
pub mod filters;
pub mod row;
pub mod table;

use table::Table;

fn main() {
    let data = data::generate_data::<u32, 3>(10);

    let row_table = row::RowTable::<u32, 3>::new(data);

    let mut filters: Vec<Box<dyn filters::Filter<u32, u32>>> = Vec::new();
    filters.push(Box::new(filters::Equal { index: 0, value: 5 }));

    let result = row_table.query([0, 1], filters);

    println!("Results:");
    for entry in &result {
        println!("{:?}", entry);
    }

    println!("Table:");
    row_table.print();
}
