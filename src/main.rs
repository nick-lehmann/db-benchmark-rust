#![feature(int_log, stdsimd)]
pub mod column;
pub mod data;
pub mod filters;
pub mod row;
pub mod table;

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

unsafe fn calculate() {
    let ones: __m512i = _mm512_set1_epi32(1);
    let values = [1, 2, 1, 3, 1, 4, 1, 5, 1, 6, 1, 7, 1, 8, 1, 9];

    let value_register = _mm512_load_epi32(&values[0]);
    let mask = _mm512_int2mask(i32::MAX);

    let result = _mm512_mask_cmp_epi32_mask::<_MM_CMPINT_EQ>(mask, ones, value_register);
    println!("{:#016b}", result);
}

fn main() {
    if is_x86_feature_detected!("avx512f")
        && is_x86_feature_detected!("avx512cd")
        && is_x86_feature_detected!("avx512er")
        && is_x86_feature_detected!("avx512pf")
    {
        // Safe because we already checked that we have
        // AVX instruction set.
        unsafe { calculate() }
    } else {
        panic!("AVX is not supported");
    }
}
