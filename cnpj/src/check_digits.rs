use std::fmt::Display;

use crate::{Error, InvalidChar, UncheckedCNPJ};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CheckDigits([u8; Self::LENGTH]);
impl CheckDigits {
    pub const LENGTH: usize = 2;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut count = 0;
        let mut bytes = [b'\0'; Self::LENGTH];

        for (index, char) in iter.into_iter().enumerate() {
            if char.is_ascii_digit() {
                if count < Self::LENGTH {
                    bytes[count] = char as u8;
                    count += 1;
                } else {
                    return Err(Error::WrongNumberOfDigits);
                }
            } else if !matches!(char, '.' | '/' | '-') {
                return Err(Error::InvalidCheckDigitChar(InvalidChar { char, index }));
            }
        }

        if count != Self::LENGTH {
            return Err(Error::WrongNumberOfDigits);
        }

        Ok(CheckDigits(bytes))
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
    fn from_iter() {
        for input in ["35", "-35", "35."] {
            assert_eq!(
                CheckDigits::from_iter(input.chars()),
                Ok(CheckDigits([b'3', b'5']))
            );
        }

        for input in ["3", "350"] {
            assert_eq!(
                CheckDigits::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits),
                "{input}"
            );
        }

        assert_eq!(
            CheckDigits::from_iter("f5".chars()),
            Err(Error::InvalidCheckDigitChar(crate::InvalidChar {
                char: 'f',
                index: 0
            }))
        );
    }
}
