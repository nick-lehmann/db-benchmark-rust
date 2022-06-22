#![allow(unused_variables, dead_code)]
#![feature(int_log, stdsimd)]
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
    let ones: __m512i = _mm512_set1_epi64(1);

    let values: [i64; 8] = [1, 2, 3, 4, 5, 6, 7, 8];
    let b: Aligned<A64, [i64; 8]> = Aligned(values);

    println!("Alignment of values: {}", std::mem::align_of::<[i64; 16]>());

    let value_register = _mm512_load_epi64((values).as_ptr());
    let result = _mm512_mask_cmp_epi64_mask::<_MM_CMPINT_EQ>(0b1111_1111, ones, value_register);

    println!("Result");
    println!("{:#08b}", result);
}

fn main() {
    if is_x86_feature_detected!("avx512f") && is_x86_feature_detected!("avx512cd") {
        // Safe because we already checked that we have
        // AVX instruction set.
        unsafe { calculate() }
    } else {
        panic!("AVX is not supported");
    }
}
