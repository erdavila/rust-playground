use std::{fmt::Display, str::FromStr};

use crate::{parser::Parser, CheckDigits, Error, InvalidChar, CPF};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UncheckedCPF(pub(crate) [u8; Self::LENGTH]);
impl UncheckedCPF {
    pub const LENGTH: usize = 9;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut iter = iter.into_iter().enumerate();

        let output = UncheckedCPFParser::parse(&mut iter)?;
        UncheckedCPFParser::ensure_all_consumed(&mut iter)?;

        Ok(output)
    }

    #[must_use]
    pub fn calculate_check_digits(self) -> CheckDigits {
        CheckDigits::from(self)
    }

    #[must_use]
    pub fn checked(self) -> CPF {
        self.with_check_digits()
    }

    #[must_use]
    pub fn with_check_digits(self) -> CPF {
        todo!()
    }

    #[must_use]
    pub fn char(self, index: usize) -> char {
        todo!()
    }

    pub fn chars(self) -> [char; Self::LENGTH] {
        todo!()
    }
}
impl FromStr for UncheckedCPF {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
impl TryFrom<&str> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<String> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Error> {
        todo!()
    }
}
impl Display for UncheckedCPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

pub(crate) struct UncheckedCPFParser;
impl Parser<{ UncheckedCPF::LENGTH }> for UncheckedCPFParser {
    type Output = UncheckedCPF;

    fn is_digit(char: char) -> bool {
        char.is_ascii_digit()
    }

    fn invalid_char_error(invalid_char: InvalidChar) -> Error {
        Error::InvalidChar(invalid_char)
    }

    fn to_output(bytes: [u8; UncheckedCPF::LENGTH]) -> Self::Output {
        UncheckedCPF(bytes)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    static FORMATTED_STR: &str = "111.444.777";
    static RAW_STR: &str = "111444777";
    pub(crate) static BYTES: [u8; UncheckedCPF::LENGTH] =
        [b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7'];

    #[test]
    fn from_iter() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(
                UncheckedCPF::from_iter(input.chars()),
                Ok(UncheckedCPF(BYTES)),
                "{input}"
            );
        }

        for input in ["111.444.77", "111.444.777-3"] {
            assert_eq!(
                UncheckedCPF::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits)
            );
        }

        assert_eq!(
            UncheckedCPF::from_iter("111,444.777".chars()),
            Err(Error::InvalidChar(InvalidChar {
                char: ',',
                index: 3
            }))
        );
    }
}
