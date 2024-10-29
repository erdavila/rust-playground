use std::fmt::Display;

use crate::{Error, UncheckedCNPJ};

pub struct CheckDigits([u8; Self::LENGTH]);
impl CheckDigits {
    pub const LENGTH: usize = 2;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
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
impl From<UncheckedCNPJ> for CheckDigits {
    fn from(value: UncheckedCNPJ) -> Self {
        todo!()
    }
}
impl TryFrom<&str> for CheckDigits {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<String> for CheckDigits {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for CheckDigits {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl Display for CheckDigits {
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
