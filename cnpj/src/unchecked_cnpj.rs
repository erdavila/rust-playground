use std::fmt::Display;

use crate::{CheckDigits, Error, InvalidChar, CNPJ};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UncheckedCNPJ(pub(crate) [u8; Self::LENGTH]);
impl UncheckedCNPJ {
    pub const LENGTH: usize = 12;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut count = 0;
        let mut bytes = [b'\0'; Self::LENGTH];

        for (index, char) in iter.into_iter().enumerate() {
            if char.is_ascii_alphanumeric() {
                if count < Self::LENGTH {
                    bytes[count] = char.to_ascii_uppercase() as u8;
                    count += 1;
                } else {
                    return Err(Error::WrongNumberOfDigits);
                }
            } else if !matches!(char, '.' | '/' | '-') {
                return Err(Error::InvalidChar(InvalidChar { char, index }));
            }
        }

        if count != Self::LENGTH {
            return Err(Error::WrongNumberOfDigits);
        }

        Ok(UncheckedCNPJ(bytes))
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
pub(crate) mod tests {
    use crate::InvalidChar;

    use super::*;

    pub(crate) static BYTES: [u8; UncheckedCNPJ::LENGTH] = [
        b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E',
    ];

    #[test]
    fn from_iter() {
        for input in ["12.AbC.345/01De", "12AbC34501De"] {
            assert_eq!(
                UncheckedCNPJ::from_iter(input.chars()),
                Ok(UncheckedCNPJ(BYTES)),
                "{input}"
            );
        }

        for input in ["12.AbC.345/01D", "12AbC34501De-3"] {
            assert_eq!(
                UncheckedCNPJ::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits)
            );
        }

        assert_eq!(
            UncheckedCNPJ::from_iter("12.AbC.345|01De".chars()),
            Err(Error::InvalidChar(InvalidChar {
                char: '|',
                index: 10
            }))
        );
    }
}
