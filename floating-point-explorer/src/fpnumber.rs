use std::fmt::Display;
use std::num::{FpCategory, ParseFloatError, ParseIntError};
use std::ops::{Div, Mul};
use std::str::FromStr;

pub trait FloatingPointNumber: Copy + Display
where
    Self: Div<Self, Output = Self>,
    Self: Mul<Self, Output = Self>,
    Self: FromStr<Err = ParseFloatError>,
{
    const BITS: usize;
    const BYTES: usize = Self::BITS / 8;
    const EXPONENT_BITS: usize;
    const FRACTION_BITS: usize = Self::BITS - Self::EXPONENT_BITS - 1;
    const EXPONENT_BIAS: i128 = (2_i128 << (Self::EXPONENT_BITS - 2)) - 1;

    const INFINITY: Self;
    const NEG_INFINITY: Self;
    const NAN: Self;

    fn from_u128(value: u128) -> Self;
    fn from_be_bytes(bytes: &[u8]) -> Self;
    fn from_bits_from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError>;
    fn to_bits(self) -> u128;
    fn to_be_bytes(self) -> Vec<u8>;
    fn classify(self) -> FpCategory;
    fn powi(self, n: i32) -> Self;
}

macro_rules! impl_floating_point_number {
    ($fp_type:ident, $u_type:ident, $bits:literal, $exponent_bits:literal) => {
        impl FloatingPointNumber for $fp_type {
            const BITS: usize = $bits;
            const EXPONENT_BITS: usize = $exponent_bits;

            const INFINITY: Self = $fp_type::INFINITY;
            const NEG_INFINITY: Self = $fp_type::NEG_INFINITY;
            const NAN: Self = $fp_type::NAN;

            fn from_u128(value: u128) -> Self {
                value as $fp_type
            }

            fn from_be_bytes(bytes: &[u8]) -> Self {
                assert_eq!(bytes.len(), Self::BYTES);
                $fp_type::from_be_bytes(std::array::from_fn(|i| bytes[i]))
            }

            fn from_bits_from_str_radix(src: &str, radix: u32) -> Result<Self, ParseIntError> {
                $u_type::from_str_radix(src, radix).map($fp_type::from_bits)
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

impl_floating_point_number!(f32, u32, 32, 8);
impl_floating_point_number!(f64, u64, 64, 11);
