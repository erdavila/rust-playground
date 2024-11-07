use std::{fmt::Display, str::FromStr};

use crate::{CheckDigits, Error, UncheckedCPF};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CPF(pub(crate) [u8; Self::LENGTH]);
impl CPF {
    pub const LENGTH: usize = UncheckedCPF::LENGTH + CheckDigits::LENGTH;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        todo!()
    }

    #[must_use]
    pub fn unchecked(self) -> UncheckedCPF {
        self.without_check_digits()
    }

    #[must_use]
    pub fn without_check_digits(self) -> UncheckedCPF {
        todo!()
    }

    #[must_use]
    pub fn check_digits(self) -> CheckDigits {
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
impl FromStr for CPF {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
impl TryFrom<&str> for CPF {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<String> for CPF {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for CPF {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Error> {
        todo!()
    }
}
impl Display for CPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) static BYTES: [u8; CPF::LENGTH] = [
        b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7', b'3', b'5',
    ];

    #[test]
    fn test() {
        //
    }
}
