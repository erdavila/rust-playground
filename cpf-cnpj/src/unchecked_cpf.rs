use std::{fmt::Display, str::FromStr};

use crate::{parser::Parser, CheckDigits, Error, InvalidChar, CPF};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UncheckedCPF(pub(crate) [u8; Self::LENGTH]);
impl UncheckedCPF {
    pub const LENGTH: usize = 9;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        todo!()
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
        todo!()
    }

    fn invalid_char_error(invalid_char: InvalidChar) -> Error {
        todo!()
    }

    fn to_output(bytes: [u8; UncheckedCPF::LENGTH]) -> Self::Output {
        todo!()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) static BYTES: [u8; UncheckedCPF::LENGTH] =
        [b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7'];

    #[test]
    fn test() {
        //
    }
}
