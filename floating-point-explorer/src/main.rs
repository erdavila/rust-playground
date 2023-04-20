use std::fmt::Display;
use std::num::FpCategory;

use args::WrappedFloatingPointNumber;
use fpnumber::FloatingPointNumber;

use crate::ansi::{
    bold, exponent_color, fraction_color, normal_color, sign_color, subnormal_color,
};
use crate::bit_groups::{BitGroups, Colorizer};
use crate::bits::Bits;

mod ansi;
mod args;
mod bit_groups;
mod bits;
mod fpnumber;

const CATEGORY_NORMAL: &str = "NORMAL";
const CATEGORY_SUBNORMAL: &str = "SUBNORMAL";
const CATEGORY_ZERO: &str = "ZERO";
const CATEGORY_INFINITY: &str = "INFINITY";
const CATEGORY_NAN: &str = "NAN";

fn main() {
    match args::parse() {
        Ok(Some(WrappedFloatingPointNumber::SinglePrecision(n))) => explore(n),
        Ok(Some(WrappedFloatingPointNumber::DoublePrecision(n))) => explore(n),
        Ok(None) => (),
        Err(e) => {
            eprintln!("ERROR: {e}");
            eprintln!("Execute with --help for expected arguments");
        }
    }
}

fn explore<N: FloatingPointNumber>(value: N) {
    let mut bits = value.to_bits();
    let fraction_bits = Bits::extract_from(&mut bits, N::FRACTION_BITS);
    let exponent_bits = Bits::extract_from(&mut bits, N::EXPONENT_BITS);
    let sign_bits = Bits::extract_from(&mut bits, 1);

    let bytes = value.to_be_bytes();

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
    print_bytes_in_base("Hex", bytes.iter().map(|byte| format!("{byte:0>2x}")), ":");
    print_bytes_in_base("Dec", bytes.iter().map(|byte| format!("{byte:>3} ")), ",");
    println!();

    let sign = sign_color(if sign_bits.value == 0x0 { "+" } else { "-" });
    let fraction = fraction_color(ensure_dot(
        N::from_u128(fraction_bits.value) / N::from_u128(1 << N::FRACTION_BITS),
    ));

    let value_composition = |exponent, int| {
        format!(
            "{value} = {value_exp} = (-1)^{sign} x 2^{exponent} x ({int} + {fraction})",
            value = bold(ensure_dot(value)),
            value_exp = bold(format!("{value:e}")),
            sign = sign_color(sign_bits.value),
        )
    };

    let (category, exponent, value) = if exponent_bits.value == 0x00 {
        if fraction_bits.value == 0 {
            assert_eq!(value.classify(), FpCategory::Zero);
            let category = CATEGORY_ZERO.to_string();
            let exponent = format!(
                "{} or {}",
                exponent_color(CATEGORY_ZERO),
                CATEGORY_SUBNORMAL
            );
            let value = bold(ensure_dot(ensure_sign(value)));

            (category, exponent, value)
        } else {
            assert_eq!(value.classify(), FpCategory::Subnormal);
            let category = subnormal_color(CATEGORY_SUBNORMAL);
            let exponent = format!(
                "{} or {}",
                CATEGORY_ZERO,
                exponent_color(CATEGORY_SUBNORMAL)
            );
            let value =
                value_composition(subnormal_color(1 - N::EXPONENT_BIAS), subnormal_color(0));

            (category, exponent, value)
        }
    } else if exponent_bits.value == Bits::mask(exponent_bits.length) {
        if fraction_bits.value == 0 {
            assert_eq!(value.classify(), FpCategory::Infinite);
            let category = CATEGORY_INFINITY.to_string();
            let exponent = format!("{} or {}", exponent_color(CATEGORY_INFINITY), CATEGORY_NAN);
            let value = bold(ensure_sign(value));

            (category, exponent, value)
        } else {
            assert_eq!(value.classify(), FpCategory::Nan);
            let category = CATEGORY_NAN.to_string();
            let exponent = format!("{} or {}", CATEGORY_INFINITY, exponent_color(CATEGORY_NAN));
            let value = bold(value);

            (category, exponent, value)
        }
    } else {
        assert_eq!(value.classify(), FpCategory::Normal);
        let category = normal_color(CATEGORY_NORMAL);
        let exponent_value = exponent_color(exponent_bits.value as i128 - N::EXPONENT_BIAS);
        let exponent = format!(
            "{} - {} = {}",
            exponent_color(exponent_bits.value),
            N::EXPONENT_BIAS,
            exponent_value,
        );
        let value = value_composition(exponent_value, normal_color(1));

        (category, exponent, value)
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
