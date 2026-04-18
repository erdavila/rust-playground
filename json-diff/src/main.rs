use std::{env, fs, io};

use anyhow::{Result, bail};
use serde_json::from_str;

use crate::compare::compare;
use crate::token::writer::TokenWriter;

mod compare;
mod comparison;
mod line_diff;
mod token;

fn main() -> Result<()> {
    let mut left_file = None;
    let mut right_file = None;

    for arg in env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: json-diff <LEFT_FILE> <RIGHT_FILE>");
                return Ok(());
            }
            _ => {
                if left_file.is_none() {
                    left_file = Some(arg);
                } else if right_file.is_none() {
                    right_file = Some(arg);
                } else {
                    bail!("Too many arguments");
                }
            }
        }
    }

    let (Some(left_file), Some(right_file)) = (left_file, right_file) else {
        bail!("Not enough arguments");
    };

    let left = from_str(&fs::read_to_string(left_file)?)?;
    let right = from_str(&fs::read_to_string(right_file)?)?;

    let comparison = compare(left, right);
    line_diff::tokenize(TokenWriter::new(io::stdout()), comparison)?;

    Ok(())
}
