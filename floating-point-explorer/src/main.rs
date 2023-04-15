use std::fmt::Display;
use std::num::FpCategory;

use crate::ansi::{
    bold, exponent_color, fraction_color, normal_color, sign_color, subnormal_color,
};
use crate::bit_groups::{BitGroups, Colorizer};
use crate::bits::Bits;

mod ansi;
mod bit_groups;
mod bits;

const CATEGORY_NORMAL: &str = "NORMAL";
const CATEGORY_SUBNORMAL: &str = "SUBNORMAL";
const CATEGORY_ZERO: &str = "ZERO";
const CATEGORY_INFINITY: &str = "INFINITY";
const CATEGORY_NAN: &str = "NAN";

fn main() {
    // NORMAL
    // let value: f64 = 1.0;
    // let value: f64 = -1.0;
    // let value: f64 = 3.0;
    // let value: f64 = 0.123456789;
    // let value: f64 = 12345.67890;
    // let value: f64 = f64::MAX;
    // let value: f64 = f64::from_bits(0x3FD5_5555_5555_5555);
    let value: f64 = std::f64::consts::PI;

    // ZERO
    // let value: f64 = 0.0;
    // let value: f64 = -0.0;

    // INFINITY
    // let value: f64 = f64::INFINITY;
    // let value: f64 = f64::NEG_INFINITY;

    // NAN
    // let value: f64 = f64::NAN;
    // let value: f64 = -f64::NAN;

    // SUBNORMAL
    // let value: f64 = f64::from_bits(0x01);
    // let value: f64 = f64::from_bits(0x000F_FFFF_FFFF_FFFF);

    let mut bits = value.to_bits() as u128;
    let fraction_bits = Bits::extract_from(&mut bits, 52);
    let exponent_bits = Bits::extract_from(&mut bits, 11);
    let sign_bits = Bits::extract_from(&mut bits, 1);

    print_bytes_in_base(
        "Bin",
        BitGroups::new(
            [
                (sign_bits.to_string(), sign_color as Colorizer),
                (exponent_bits.to_string(), exponent_color),
                (fraction_bits.to_string(), fraction_color),
            ]
            .into_iter(),
        ),
        ":",
    );
    print_bytes_in_base(
        "Hex",
        value.to_be_bytes().map(|byte| format!("{byte:0>2x}")),
        ":",
    );
    print_bytes_in_base(
        "Dec",
        value.to_be_bytes().map(|byte| format!("{byte:>3} ")),
        ",",
    );
    println!();

    let sign = sign_color(if sign_bits.value == 0x0 { "+" } else { "-" });
    let fraction = fraction_color(ensure_dot(fraction_bits.value as f64 / 2_f64.powi(52)));

    let value_composition = |exponent, int| {
        format!(
            "{value} = (-1)^{sign} x 2^{exponent} x ({int} + {fraction})",
            value = bold(ensure_dot(value)),
            sign = sign_color(sign_bits.value),
        )
    };

    let (category, exponent, value) = match (exponent_bits.value, fraction_bits.value) {
        (0x000, 0) => {
            assert_eq!(value.classify(), FpCategory::Zero);
            let category = CATEGORY_ZERO.to_string();
            let exponent = format!(
                "{} or {}",
                exponent_color(CATEGORY_ZERO),
                CATEGORY_SUBNORMAL
            );
            let value = bold(ensure_dot(ensure_sign(value)));

            (category, exponent, value)
        }
        (0x000, _) => {
            assert_eq!(value.classify(), FpCategory::Subnormal);
            let category = subnormal_color(CATEGORY_SUBNORMAL);
            let exponent = format!(
                "{} or {}",
                CATEGORY_ZERO,
                exponent_color(CATEGORY_SUBNORMAL)
            );
            let value = value_composition(subnormal_color(-1022), subnormal_color(0));

            (category, exponent, value)
        }
        (0x7FF, 0) => {
            assert_eq!(value.classify(), FpCategory::Infinite);
            let category = CATEGORY_INFINITY.to_string();
            let exponent = format!("{} or {}", exponent_color(CATEGORY_INFINITY), CATEGORY_NAN);
            let value = bold(ensure_sign(value));

            (category, exponent, value)
        }
        (0x7FF, _) => {
            assert_eq!(value.classify(), FpCategory::Nan);
            let category = CATEGORY_NAN.to_string();
            let exponent = format!("{} or {}", CATEGORY_INFINITY, exponent_color(CATEGORY_NAN));
            let value = bold(value);

            (category, exponent, value)
        }
        _ => {
            assert_eq!(value.classify(), FpCategory::Normal);
            let category = normal_color(CATEGORY_NORMAL);
            let exponent_value = exponent_color(exponent_bits.value as i128 - 1023);
            let exponent = format!(
                "{} - 1023 = {}",
                exponent_color(exponent_bits.value),
                exponent_value
            );
            let value = value_composition(exponent_value, normal_color(1));

            (category, exponent, value)
        }
    };

    println!("Category: {category}");
    println!();

    println!("{}: {sign}", sign_color("Sign"));
    println!("{}: {exponent}", exponent_color("Exponent"));
    println!("{}: {fraction}", fraction_color("Fraction"));
    println!();

    println!("Value: {value}");
}

fn print_bytes_in_base(title: &str, bytes: impl IntoIterator<Item = String>, separator: &str) {
    println!(
        "{title}: {}",
        bytes
            .into_iter()
            .map(|byte| format!("{byte:^8}"))
            .collect::<Vec<_>>()
            .join(separator)
    );
}

fn ensure_dot<D: Display>(d: D) -> String {
    let s = d.to_string();
    if s.contains('.') {
        s
    } else {
        s + ".0"
    }
}

fn ensure_sign<D: Display>(d: D) -> String {
    let s = d.to_string();
    if s.starts_with(['+', '-']) {
        s
    } else {
        format!("+{s}")
    }
}
