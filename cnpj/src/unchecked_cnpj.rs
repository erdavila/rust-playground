use std::{array, fmt::Display};

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
        let check_digits = self.calculate_check_digits();
        let bytes = array::from_fn(|i| {
            if i < Self::LENGTH {
                self.0[i]
            } else {
                check_digits.0[i - Self::LENGTH]
            }
        });

        CNPJ(bytes)
    }

    #[must_use]
    pub fn char(self, index: usize) -> char {
        self.0[index].into()
    }

    pub fn chars(self) -> [char; Self::LENGTH] {
        self.0.map(Into::into)
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

    #[test]
    fn with_check_digits() {
        let unchecked_cnpj = UncheckedCNPJ(BYTES);

        assert_eq!(
            unchecked_cnpj.with_check_digits(),
            CNPJ([
                b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E', b'3', b'5'
            ])
        );
    }

    #[test]
    fn char() {
        let unchecked_cnpj = UncheckedCNPJ(BYTES);

        assert_eq!(unchecked_cnpj.char(0), '1');
        assert_eq!(unchecked_cnpj.char(1), '2');
        assert_eq!(unchecked_cnpj.char(2), 'A');
        assert_eq!(unchecked_cnpj.char(3), 'B');
        assert_eq!(unchecked_cnpj.char(4), 'C');
        assert_eq!(unchecked_cnpj.char(5), '3');
        assert_eq!(unchecked_cnpj.char(6), '4');
        assert_eq!(unchecked_cnpj.char(7), '5');
        assert_eq!(unchecked_cnpj.char(8), '0');
        assert_eq!(unchecked_cnpj.char(9), '1');
        assert_eq!(unchecked_cnpj.char(10), 'D');
        assert_eq!(unchecked_cnpj.char(11), 'E');
    }

    #[test]
    fn chars() {
        let unchecked_cnpj = UncheckedCNPJ(BYTES);

        assert_eq!(
            unchecked_cnpj.chars(),
            ['1', '2', 'A', 'B', 'C', '3', '4', '5', '0', '1', 'D', 'E']
        );
    }
}
