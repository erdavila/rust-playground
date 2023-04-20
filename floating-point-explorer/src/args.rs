use std::env;
use std::num::{ParseFloatError, ParseIntError};

use crate::fpnumber::FloatingPointNumber;

pub enum WrappedFloatingPointNumber {
    SinglePrecision(f32),
    DoublePrecision(f64),
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
#[allow(clippy::enum_variant_names)]
pub enum Error {
    InvalidArgumentsCount {
        found: usize,
        expected: usize,
    },
    InvalidPrecision,
    InvalidLength {
        found: usize,
        expected: usize,
        base: u32,
    },
    InvalidNumberOfBytes {
        found: usize,
        expected: usize,
    },
    ParseIntError(ParseIntError),
    ParseFloatError(ParseFloatError),
    UnrecognizedNumberFormat,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::InvalidArgumentsCount { found, expected } => {
                write!(
                    f,
                    "Invalid number of arguments. Found: {found}. Expected: {expected}"
                )
            }
            Error::InvalidPrecision => write!(f, "Invalid precision argument"),
            Error::InvalidLength {
                found,
                expected,
                base,
            } => write!(
                f,
                "Invalid length for number in base {base}. Found: {found}. Expected: {expected}"
            ),
            Error::InvalidNumberOfBytes { found, expected } => write!(
                f,
                "Invalid number of bytes values. Found: {found}. Expected: {expected}"
            ),
            Error::ParseIntError(e) => write!(f, "Can't parse number. {e}"),
            Error::ParseFloatError(e) => write!(f, "Can't parse number. {e}"),
            Error::UnrecognizedNumberFormat => write!(f, "Unrecognized number format"),
        }
    }
}

impl From<ParseIntError> for Error {
    fn from(value: ParseIntError) -> Self {
        Error::ParseIntError(value)
    }
}

impl From<ParseFloatError> for Error {
    fn from(value: ParseFloatError) -> Self {
        Error::ParseFloatError(value)
    }
}

pub fn parse() -> Result<Option<WrappedFloatingPointNumber>> {
    let args: Vec<_> = env::args().skip(1).collect();

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return Ok(None);
    }

    const EXPECTED_ARGUMENTS_COUNT: usize = 2;

    if args.len() != EXPECTED_ARGUMENTS_COUNT {
        return Err(Error::InvalidArgumentsCount {
            found: args.len(),
            expected: EXPECTED_ARGUMENTS_COUNT,
        });
    }

    match &args[0].to_lowercase()[..] {
        "single" => {
            let n: f32 = parse_value(&args[1].to_ascii_lowercase())?;
            Ok(Some(WrappedFloatingPointNumber::SinglePrecision(n)))
        }
        "double" => {
            let n: f64 = parse_value(&args[1].to_ascii_lowercase())?;
            Ok(Some(WrappedFloatingPointNumber::DoublePrecision(n)))
        }
        other => {
            println!("OTHER! {}", other);
            Err(Error::InvalidPrecision)
        }
    }
}

fn parse_value<N: FloatingPointNumber>(arg: &str) -> Result<N> {
    for f in [
        parse_nan,
        parse_infinity,
        parse_binary,
        parse_hexadecimal,
        parse_decimal_bytes,
        parse_composed,
    ] {
        match f(arg)? {
            Some(n) => return Ok(n),
            None => continue,
        }
    }

    Err(Error::UnrecognizedNumberFormat)
}

fn parse_nan<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    Ok((arg == "nan").then_some(N::NAN))
}

fn parse_infinity<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    let (negative, arg) = if let Some(arg) = arg.strip_prefix('-') {
        (true, arg)
    } else if let Some(arg) = arg.strip_prefix('+') {
        (false, arg)
    } else {
        (false, arg)
    };

    Ok(matches!(arg, "inf" | "infinity" | "infinite").then(|| {
        if negative {
            N::NEG_INFINITY
        } else {
            N::INFINITY
        }
    }))
}

fn parse_binary<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    radix_parse(arg, ['b'], 2, N::BITS)
}

fn parse_hexadecimal<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    radix_parse(arg, ['h', 'x'], 16, N::BITS / 4)
}

fn radix_parse<N: FloatingPointNumber, const P: usize>(
    arg: &str,
    prefixes: [char; P],
    radix: u32,
    len: usize,
) -> Result<Option<N>> {
    if let Some(arg) = arg.strip_prefix(prefixes) {
        let arg = arg.replace(':', "");
        if arg.len() != len {
            return Err(Error::InvalidLength {
                found: arg.len(),
                expected: len,
                base: radix,
            });
        }
        Ok(Some(N::from_bits_from_str_radix(&arg, radix)?))
    } else {
        Ok(None)
    }
}

fn parse_decimal_bytes<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    let bytes: Vec<_> = arg.split(',').collect();
    let len = bytes.len();
    if len == 1 {
        Ok(None)
    } else if len == N::BYTES {
        let bytes: std::result::Result<Vec<_>, _> =
            bytes.into_iter().map(|b| b.parse::<u8>()).collect();
        let bytes = bytes?;
        let n = N::from_be_bytes(&bytes);
        Ok(Some(n))
    } else {
        Err(Error::InvalidNumberOfBytes {
            found: len,
            expected: N::BYTES,
        })
    }
}

fn parse_composed<N: FloatingPointNumber>(arg: &str) -> Result<Option<N>> {
    match split(arg, ['e', 'b']) {
        Some((mantissa, base_char, exponent)) => {
            if let (Some(mantissa), Some(exponent)) =
                (parse_mantissa::<N>(mantissa), parse_exponent(exponent))
            {
                let n = match base_char {
                    'e' => arg.parse::<N>().unwrap(),
                    'b' => mantissa * N::from_u128(2).powi(exponent),
                    _ => unreachable!(),
                };
                Ok(Some(n))
            } else {
                Ok(None)
            }
        }
        None => Ok(parse_mantissa(arg)),
    }
}

fn parse_mantissa<N: FloatingPointNumber>(mantissa: &str) -> Option<N> {
    let is_mantissa = {
        let unsigned_mantissa = strip_sign(mantissa);
        let (int, fraction) = match split(unsigned_mantissa, ['.']) {
            Some((int, _, fraction)) => (int, fraction),
            None => (unsigned_mantissa, ""),
        };
        (!int.is_empty() || !fraction.is_empty())
            && has_digits_only(int)
            && has_digits_only(fraction)
    };

    is_mantissa.then(|| mantissa.parse::<N>().unwrap())
}

fn parse_exponent(exponent: &str) -> Option<i32> {
    let is_exponent = {
        let unsigned_exponent = strip_sign(exponent);
        !unsigned_exponent.is_empty() && has_digits_only(unsigned_exponent)
    };

    is_exponent.then(|| exponent.parse().unwrap())
}

fn split<const N: usize>(arg: &str, separators: [char; N]) -> Option<(&str, char, &str)> {
    arg.find(separators).map(|index| {
        let before = &arg[..index];
        let separator = arg.chars().nth(index).unwrap();
        let after = &arg[(index + 1)..];
        (before, separator, after)
    })
}

fn strip_sign(arg: &str) -> &str {
    arg.strip_prefix(['+', '-']).unwrap_or(arg)
}

fn has_digits_only(arg: &str) -> bool {
    arg.chars().all(|char| ('0'..='9').contains(&char))
}

fn print_help() {
    println!("Expected arguments: PRECISION VALUE");
    println!("  The arguments are parsed in a case-insensitive manner.");
    println!();
    println!("PRECISION");
    println!();
    println!("  'single' - for single-precision, 32 bits, floating points (IEEE 754 binary32).");
    println!("  'double' - for double-precision, 64 bits, floating points (IEEE 754 binary64).");
    println!();
    println!("VALUE");
    println!();
    println!("  Binary digits - the character 'b' followed by the binary digits.");
    println!("    Exactly 32 binary digits are expected for single-precision values.");
    println!("    Exactly 64 binary digits are expected for double-precision values.");
    println!("    The separator ':' is used only for visual aid, and is ignored during parsing.");
    println!("    Examples:");
    println!("      single b:1001:0110:1011:0110:0010:0101:1010:0101");
    println!(
        "      double b:10010110:10110110:00100101:10100101:10010110:10110110:00100101:10100101"
    );
    println!();
    println!("  Hexadecimal digits - the character 'h' or 'x' followed by the hexadecimal digits.");
    println!("    Exactly 8 hexadecimal digits are expected for single-precision values.");
    println!("    Exactly 16 hexadecimal digits are expected for double-precision values.");
    println!("    The separator ':' is used only for visual aid, and is ignored during parsing.");
    println!("    Examples:");
    println!("      single h:96b6:25a5");
    println!("      double h:4009:21FB:5444:2D18");
    println!();
    println!("  Decimal bytes - comma-delimited bytes values in decimal representation.");
    println!("    Exactly 4 byte values are expected for single-precision values.");
    println!("    Exactly 8 byte values are expected for double-precision values.");
    println!("    Examples:");
    println!("      single 150,182,37,165");
    println!("      double 150,182,37,165,150,182,37,165");
    println!();
    println!("  Mantissa and exponent - a mantissa value is optionally followed by the base indicator and the exponent value.");
    println!("    The mantissa value is composed by an optional sign, followed by digits possibly containing a decimal point.");
    println!("    The base indicator is the character 'e' for base 10 and 'b' for base 2.");
    println!("    The exponent is an optional sign followed by a natural number.");
    println!("    Examples:");
    println!("      single 1");
    println!("      double -1.");
    println!("      single .2");
    println!("      double -1.2");
    println!("      single 1.2e3  # meaning 1.2 x 10^3 = 1200.0");
    println!("      double -1.2b-3  # meaning -1.2 x 2^-3 = -0.15");
    println!();
    println!("  Infinity - an optional sign followed by 'inf', 'infinity', or 'infinite'.");
    println!("    Examples:");
    println!("      single inf");
    println!("      single -inf");
    println!("      double +infinity");
    println!();
    println!("  Not-a-number - simply the string 'NaN'.");
    println!("    Examples:");
    println!("      single NAN");
    println!("      double nan");
}
