use std::fmt::Display;
use std::num::FpCategory;
use std::ops::Div;

pub trait FloatingPointNumber: Copy + Display + Div<Self, Output = Self> {
    const BITS: usize;
    const EXPONENT_BITS: usize;
    const FRACTION_BITS: usize = Self::BITS - Self::EXPONENT_BITS - 1;
    const EXPONENT_BIAS: i128 = (2_i128 << (Self::EXPONENT_BITS - 2)) - 1;

    fn from_u128(value: u128) -> Self;
    fn to_bits(self) -> u128;
    fn to_be_bytes(self) -> Vec<u8>;
    fn classify(self) -> FpCategory;
    fn powi(self, n: i32) -> Self;
}

macro_rules! impl_floating_point_number {
    ($fp_type:ident, $bits:literal, $exponent_bits:literal) => {
        impl FloatingPointNumber for $fp_type {
            const BITS: usize = $bits;
            const EXPONENT_BITS: usize = $exponent_bits;

            fn from_u128(value: u128) -> Self {
                value as $fp_type
            }

            fn to_bits(self) -> u128 {
                $fp_type::to_bits(self) as u128
            }

            fn to_be_bytes(self) -> Vec<u8> {
                Vec::from($fp_type::to_be_bytes(self))
            }

            fn classify(self) -> FpCategory {
                $fp_type::classify(self)
            }

            fn powi(self, n: i32) -> Self {
                $fp_type::powi(self, n)
            }
        }
    };
}

impl_floating_point_number!(f32, 32, 8);
impl_floating_point_number!(f64, 64, 11);
