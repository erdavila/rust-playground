use std::fmt::Display;

use crate::{CheckDigits, Error, UncheckedCNPJ};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CNPJ([u8; Self::LENGTH]);
impl CNPJ {
    pub const LENGTH: usize = UncheckedCNPJ::LENGTH + CheckDigits::LENGTH;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        todo!()
    }

    #[must_use]
    pub fn unchecked(self) -> UncheckedCNPJ {
        self.without_check_digits()
    }

    #[must_use]
    pub fn without_check_digits(self) -> UncheckedCNPJ {
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
impl TryFrom<&str> for CNPJ {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<String> for CNPJ {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for CNPJ {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Error> {
        todo!()
    }
}
impl Display for CNPJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
