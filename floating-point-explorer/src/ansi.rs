use std::fmt::Display;

pub fn sign_color<D: Display>(d: D) -> String {
    color(d, 34)
}

pub fn exponent_color<D: Display>(d: D) -> String {
    color(d, 32)
}

pub fn fraction_color<D: Display>(d: D) -> String {
    color(d, 31)
}

pub fn normal_color<D: Display>(d: D) -> String {
    color(d, 33)
}

pub fn subnormal_color<D: Display>(d: D) -> String {
    color(d, 35)
}

fn color<D: Display>(d: D, code: u8) -> String {
    format(d, code, 39)
}

pub fn bold<D: Display>(d: D) -> String {
    format(d, 1, 22)
}

fn format<D: Display>(d: D, begin: u8, end: u8) -> String {
    format!("{}{d}{}", escape(begin), escape(end))
}

fn escape(code: u8) -> String {
    format!("\x1b[{code}m")
}
