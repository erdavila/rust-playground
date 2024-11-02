use std::fmt::Display;

use crate::{
    parse::{CheckDigitsParser, Parser, UncheckedCNPJParser},
    CheckDigits, Error, UncheckedCNPJ,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CNPJ(pub(crate) [u8; Self::LENGTH]);
impl CNPJ {
    pub const LENGTH: usize = UncheckedCNPJ::LENGTH + CheckDigits::LENGTH;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut iter = iter.into_iter().enumerate();

        let unchecked_cnpj = UncheckedCNPJParser::parse(&mut iter)?;
        let check_digits = CheckDigitsParser::parse(&mut iter)?;
        CheckDigitsParser::ensure_all_consumed(&mut iter)?;

        if unchecked_cnpj.calculate_check_digits() != check_digits {
            return Err(Error::WrongCheckDigits);
        }

        let mut bytes = [b'\0'; Self::LENGTH];
        bytes[..UncheckedCNPJ::LENGTH].copy_from_slice(&unchecked_cnpj.0);
        bytes[UncheckedCNPJ::LENGTH..].copy_from_slice(&check_digits.0);

        Ok(CNPJ(bytes))
    }

    #[must_use]
    pub fn unchecked(self) -> UncheckedCNPJ {
        self.without_check_digits()
    }

    #[must_use]
    pub fn without_check_digits(self) -> UncheckedCNPJ {
        let mut bytes = [b'\0'; UncheckedCNPJ::LENGTH];
        bytes.copy_from_slice(&self.0[..UncheckedCNPJ::LENGTH]);
        UncheckedCNPJ(bytes)
    }

    #[must_use]
    pub fn check_digits(self) -> CheckDigits {
        let mut bytes = [b'\0'; CheckDigits::LENGTH];
        bytes.copy_from_slice(&self.0[UncheckedCNPJ::LENGTH..]);
        CheckDigits(bytes)
    }

    #[must_use]
    pub fn char(self, index: usize) -> char {
        self.0[index].into()
    }

    pub fn chars(self) -> [char; Self::LENGTH] {
        self.0.map(Into::into)
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
    use crate::{check_digits, unchecked_cnpj, InvalidChar};

    use super::*;

    static BYTES: [u8; CNPJ::LENGTH] = [
        b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E', b'3', b'5',
    ];

    #[test]
    fn from_iter() {
        for input in ["12.AbC.345/01De-35", "12ABC34501DE35"] {
            assert_eq!(CNPJ::from_iter(input.chars()), Ok(CNPJ(BYTES)));
        }

        for input in ["12.AbC.345/01De-3", "12.AbC.345/01De-350"] {
            assert_eq!(
                CNPJ::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits)
            );
        }

        assert_eq!(
            CNPJ::from_iter("12.AbC.345|01De-35".chars()),
            Err(Error::InvalidChar(InvalidChar {
                char: '|',
                index: 10
            }))
        );
        assert_eq!(
            CNPJ::from_iter("12.AbC.345/01De-f5".chars()),
            Err(Error::InvalidCheckDigitChar(InvalidChar {
                char: 'f',
                index: 16
            }))
        );

        for input in ["12.AbC.345/01De-05", "12.AbC.345/01De-30"] {
            assert_eq!(CNPJ::from_iter(input.chars()), Err(Error::WrongCheckDigits));
        }
    }

    #[test]
    fn without_check_digits() {
        let cnpj = CNPJ(BYTES);

        assert_eq!(
            cnpj.without_check_digits(),
            UncheckedCNPJ(unchecked_cnpj::tests::BYTES)
        );
    }

    #[test]
    fn check_digits() {
        let cnpj = CNPJ(BYTES);

        assert_eq!(cnpj.check_digits(), CheckDigits(check_digits::tests::BYTES));
    }

    #[test]
    fn char() {
        let cnpj = CNPJ(BYTES);

        assert_eq!(cnpj.char(0), '1');
        assert_eq!(cnpj.char(1), '2');
        assert_eq!(cnpj.char(2), 'A');
        assert_eq!(cnpj.char(3), 'B');
        assert_eq!(cnpj.char(4), 'C');
        assert_eq!(cnpj.char(5), '3');
        assert_eq!(cnpj.char(6), '4');
        assert_eq!(cnpj.char(7), '5');
        assert_eq!(cnpj.char(8), '0');
        assert_eq!(cnpj.char(9), '1');
        assert_eq!(cnpj.char(10), 'D');
        assert_eq!(cnpj.char(11), 'E');
        assert_eq!(cnpj.char(12), '3');
        assert_eq!(cnpj.char(13), '5');
    }

    #[test]
    fn chars() {
        let cnpj = CNPJ(BYTES);

        assert_eq!(
            cnpj.chars(),
            ['1', '2', 'A', 'B', 'C', '3', '4', '5', '0', '1', 'D', 'E', '3', '5']
        );
    }
}
