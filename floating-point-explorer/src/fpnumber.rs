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

impl FloatingPointNumber for f64 {
    const BITS: usize = 64;
    const EXPONENT_BITS: usize = 11;

    fn from_u128(value: u128) -> Self {
        value as f64
    }

    fn to_bits(self) -> u128 {
        f64::to_bits(self) as u128
    }

    fn to_be_bytes(self) -> Vec<u8> {
        Vec::from(f64::to_be_bytes(self))
    }

    fn classify(self) -> FpCategory {
        f64::classify(self)
    }

    fn powi(self, n: i32) -> Self {
        f64::powi(self, n)
    }
}
