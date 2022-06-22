#![allow(unused_imports, unused_variables, dead_code)]
use std::mem;

static PAGE_SIZE: usize = 4096;
const ATTRIBUTES: usize = 3;

#[repr(C, align(4096))]
#[derive(Debug)]
struct Page<T, const ATTRS: usize, const ROWS: usize> {
    attribute_sizes: [u16; ATTRS],
    number_of_records: u16,
    free_space: u16,

    minipages: [Minipage<T, ROWS>; ATTRS],
}

impl<T: Sized + Copy, const ATTRS: usize, const ROWS: usize> Page<T, ATTRS, ROWS> {
    pub const fn new(default_value: T) -> Self {
        Page {
            attribute_sizes: [8; ATTRS],
            number_of_records: 0,
            free_space: 100,
            minipages: [Minipage {
                data: [default_value; ROWS],
            }; ATTRS],
        }
    }
}

const fn rows_of_page<T: Sized>() -> usize {
    return (4096 - 2 * ATTRIBUTES - 2) / ATTRIBUTES / ::core::mem::size_of::<T>();
}

#[repr(C)]
#[derive(Debug)]
struct Minipage<T, const M: usize> {
    data: [T; M],
}

fn create_page<T: Sized>(cell_size: usize) {}

fn main() {
    const ROWS: usize = 10;
    let page: Page<u32, 3, ROWS> = Page::new(0u32);

    // eprintln!("{:#?} = {:#x?}", page, unsafe {
    //     ::core::slice::from_raw_parts(
    //         &page as *const _ as *const ::std::sync::atomic::AtomicU8,
    //         ::core::mem::size_of_val(&page),
    //     )
    // });
}
