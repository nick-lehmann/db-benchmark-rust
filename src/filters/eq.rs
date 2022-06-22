use super::{IndexContainer, ScalarFilter, VectorFilter};

#[cfg(target_arch = "x86")]
use std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::*;

pub struct Equal<Value> {
    pub index: usize,
    pub value: Value,

    // Note: Conditional compilation is not possible since there is no
    // target feature available for AVX512.
    // Source: https://doc.rust-lang.org/reference/attributes/codegen.html#x86-or-x86_64
    pub value_register: Option<__m512i>,
}

impl Equal<i32> {
    pub fn new(index: usize, value: i32) -> Self {
        Equal {
            index,
            value,
            value_register: match is_x86_feature_detected!("avx512f") {
                true => unsafe { Some(_mm512_set1_epi32(value)) },
                false => None,
            },
        }
    }
}

impl Equal<i64> {
    pub fn new(index: usize, value: i64) -> Self {
        Equal {
            index,
            value,
            value_register: match is_x86_feature_detected!("avx512f") {
                true => unsafe { Some(_mm512_set1_epi64(value)) },
                false => None,
            },
        }
    }
}

impl<Value> IndexContainer for Equal<Value> {
    fn index(&self) -> usize {
        self.index
    }
}

impl<Value, Input> ScalarFilter<Value, Input> for Equal<Value>
where
    Input: PartialEq<Value>,
{
    fn compare(&self, value: Input) -> bool {
        value == self.value
    }
}

impl VectorFilter<__m512i, i32, __mmask16> for Equal<i32> {
    fn compare(&self, value: __m512i, mask: __mmask16) -> __mmask16 {
        unsafe {
            _mm512_mask_cmpeq_epi32_mask(
                mask,
                value,
                self.value_register
                    .expect("Called AVX512 instruction but the instruction set is not available."),
            )
        }
    }
}

impl VectorFilter<__m512i, i64, __mmask8> for Equal<i64> {
    fn compare(&self, value: __m512i, mask: __mmask8) -> __mmask8 {
        unsafe {
            _mm512_mask_cmpeq_epi64_mask(
                mask,
                value,
                self.value_register
                    .expect("Called AVX512 instruction but the instruction set is not available."),
            )
        }
    }
}
