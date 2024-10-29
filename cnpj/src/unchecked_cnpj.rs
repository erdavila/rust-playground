use std::fmt::Display;

use crate::{CheckDigits, Error, CNPJ};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UncheckedCNPJ(pub(crate) [u8; Self::LENGTH]);
impl UncheckedCNPJ {
    pub const LENGTH: usize = 12;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        todo!()
    }

    #[must_use]
    pub fn calculate_check_digits(self) -> CheckDigits {
        CheckDigits::from(self)
    }

    #[must_use]
    pub fn checked(self) -> CNPJ {
        self.with_check_digits()
    }

    #[must_use]
    pub fn with_check_digits(self) -> CNPJ {
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
impl TryFrom<&str> for UncheckedCNPJ {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<String> for UncheckedCNPJ {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for UncheckedCNPJ {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Error> {
        todo!()
    }
}
impl Display for UncheckedCNPJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        //
    }
}
