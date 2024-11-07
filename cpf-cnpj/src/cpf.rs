use std::{fmt::Display, str::FromStr};

use crate::{
    parser::Parser, CheckDigits, CheckDigitsParser, Error, UncheckedCPF, UncheckedCPFParser,
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CPF(pub(crate) [u8; Self::LENGTH]);
impl CPF {
    pub const LENGTH: usize = UncheckedCPF::LENGTH + CheckDigits::LENGTH;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut iter = iter.into_iter().enumerate();
        let unchecked_cpf = UncheckedCPFParser::parse(&mut iter)?;
        let check_digits = CheckDigitsParser::parse(&mut iter)?;
        CheckDigitsParser::ensure_all_consumed(&mut iter)?;

        if unchecked_cpf.calculate_check_digits() != check_digits {
            return Err(Error::WrongCheckDigits);
        }

        let mut bytes = [b'\0'; Self::LENGTH];
        bytes[..UncheckedCPF::LENGTH].copy_from_slice(&unchecked_cpf.0);
        bytes[UncheckedCPF::LENGTH..].copy_from_slice(&check_digits.0);

        Ok(CPF(bytes))
    }

    #[must_use]
    pub fn unchecked(self) -> UncheckedCPF {
        self.without_check_digits()
    }

    #[must_use]
    pub fn without_check_digits(self) -> UncheckedCPF {
        let mut bytes = [b'\0'; UncheckedCPF::LENGTH];
        bytes.copy_from_slice(&self.0[..UncheckedCPF::LENGTH]);
        UncheckedCPF(bytes)
    }

    #[must_use]
    pub fn check_digits(self) -> CheckDigits {
        let mut bytes = [b'\0'; CheckDigits::LENGTH];
        bytes.copy_from_slice(&self.0[UncheckedCPF::LENGTH..]);
        CheckDigits(bytes)
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
    use crate::{check_digits, unchecked_cpf, InvalidChar};

    use super::*;

    static FORMATTED_STR: &str = "111.444.777-35";
    static RAW_STR: &str = "11144477735";
    pub(crate) static BYTES: [u8; CPF::LENGTH] = [
        b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7', b'3', b'5',
    ];

    #[test]
    fn from_iter() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(CPF::from_iter(input.chars()), Ok(CPF(BYTES)));
        }

        for input in ["111.444.777-3", "111.444.777-350"] {
            assert_eq!(
                CPF::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits)
            );
        }

        assert_eq!(
            CPF::from_iter("111,444.777-35".chars()),
            Err(Error::InvalidChar(InvalidChar {
                char: ',',
                index: 3
            }))
        );
        assert_eq!(
            CPF::from_iter("111.444.777-f5".chars()),
            Err(Error::InvalidCheckDigitChar(InvalidChar {
                char: 'f',
                index: 12
            }))
        );

        for input in ["111.444.777-05", "111.444.777-30"] {
            assert_eq!(CPF::from_iter(input.chars()), Err(Error::WrongCheckDigits));
        }
    }

    #[test]
    fn without_check_digits() {
        let cpf = CPF(BYTES);

        assert_eq!(
            cpf.without_check_digits(),
            UncheckedCPF(unchecked_cpf::tests::BYTES)
        );
    }

    #[test]
    fn check_digits() {
        let cpf = CPF(BYTES);

        assert_eq!(cpf.check_digits(), CheckDigits(check_digits::tests::BYTES));
    }
}
