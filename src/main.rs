#![allow(unused_variables, dead_code, unused_mut)]
#![feature(int_log, stdsimd, fn_traits)]
pub mod data;
pub mod filters;
pub mod tables;

// static X: Aligned<A2, [u8; 3]> = Aligned([0; 3]);

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

use aligned::{Aligned, A64};

#[repr(align(64))]
struct Align {
    data: [i64; 8], // 64 bytes
}

unsafe fn calculate() {
    let ones = _mm512_set1_epi64(1);

    let values: [i64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b: Aligned<A64, [i64; 8]> = Aligned(values);

    println!("Alignment of values: {}", std::mem::align_of::<[i64; 16]>());

    let value_register = _mm512_load_epi64((values).as_ptr());
    let result = _mm512_mask_cmp_epi64_mask::<_MM_CMPINT_EQ>(0b1111_1111, ones, value_register);

    println!("Result");
    println!("{:#08b}", result);
}

unsafe fn store() {
    let mut values = [0i64; 8];

    let ptr = values.as_mut_ptr() as *mut u8;
    let ones = _mm512_set1_epi64(1);
    let register = _mm512_set_epi64(1, 2, 3, 4, 5, 6, 7, 8);
    let mask = 0b00001111;

    _mm512_mask_compressstoreu_epi64(ptr, mask, register);

    println!("{:?}", values);
}

fn main() {
    let x = [0u32; 16];
    let v: Vec<&u32> = x.iter().collect();
    let chunk_size = 16;
    let rows = 16;

    for index in (0..rows).step_by(chunk_size) {
        println!("{}", index);
    }

    let a = [0i32; 16];
    let x = &a[0..4];
    println!("Len: {}", x.len())

    // if is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512cd") {
    //     // Safe because we already checked that we have
    //     // AVX instruction set.
    //     unsafe { store() }
    // } else {
    //     panic!("AVX is not supported");
    // }
}
