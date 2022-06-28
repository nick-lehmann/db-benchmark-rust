use super::{IndexContainer, ScalarFilter, VectorFilter};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct GreaterEqual<Value> {
    index: usize,
    value: Value,
    value_register: Option<__m512i>,
}

impl GreaterEqual<i32> {
    pub fn new(index: usize, value: i32) -> Self {
        GreaterEqual {
            index,
            value,
            value_register: match is_x86_feature_detected!("avx512f") {
                true => unsafe { Some(_mm512_set1_epi32(value)) },
                false => None,
            },
        }
    }
}

impl GreaterEqual<i64> {
    pub fn new(index: usize, value: i64) -> Self {
        GreaterEqual {
            index,
            value,
            value_register: match is_x86_feature_detected!("avx512f") {
                true => unsafe { Some(_mm512_set1_epi64(value)) },
                false => None,
            },
        }
    }
}

impl<Value> IndexContainer for GreaterEqual<Value> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<Value, Input> ScalarFilter<Value, Input> for GreaterEqual<Value>
where
    Input: PartialOrd<Value>,
{
    fn compare(&self, value: Input) -> bool {
        value >= self.value
    }
}

impl VectorFilter<__m512i, i32, __mmask16> for GreaterEqual<i32> {
    #[inline(always)]
    fn compare(&self, value: __m512i, mask: __mmask16) -> __mmask16 {
        unsafe {
            _mm512_mask_cmpge_epi32_mask(
                mask,
                value,
                self.value_register
                    .expect("Called AVX512 instruction but the instruction set is not available."),
            )
        }
    }
}

impl VectorFilter<__m512i, i64, __mmask8> for GreaterEqual<i64> {
    #[inline(always)]
    fn compare(&self, value: __m512i, mask: __mmask8) -> __mmask8 {
        unsafe {
            _mm512_mask_cmpge_epi64_mask(
                mask,
                value,
                self.value_register
                    .expect("Called AVX512 instruction but the instruction set is not available."),
            )
        }
    }
}
