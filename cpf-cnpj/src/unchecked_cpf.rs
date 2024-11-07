use std::{array, fmt::Display, str::FromStr};

use crate::{parser::Parser, CheckDigits, Error, InvalidChar, CPF};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UncheckedCPF(pub(crate) [u8; Self::LENGTH]);
impl UncheckedCPF {
    pub const LENGTH: usize = 9;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut iter = iter.into_iter().enumerate();

        let output = UncheckedCPFParser::parse(&mut iter)?;
        UncheckedCPFParser::ensure_all_consumed(&mut iter)?;

        Ok(output)
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
        let check_digits = self.calculate_check_digits();
        let bytes = array::from_fn(|i| {
            if i < Self::LENGTH {
                self.0[i]
            } else {
                check_digits.0[i - Self::LENGTH]
            }
        });

        CPF(bytes)
    }

    #[must_use]
    pub fn char(self, index: usize) -> char {
        self.0[index].into()
    }

    pub fn chars(self) -> [char; Self::LENGTH] {
        self.0.map(Into::into)
    }
}
impl FromStr for UncheckedCPF {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_iter(s.chars())
    }
}
impl TryFrom<&str> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Error> {
        Self::from_iter(value.chars())
    }
}
impl TryFrom<String> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Error> {
        Self::from_iter(value.chars())
    }
}
impl TryFrom<[char; Self::LENGTH]> for UncheckedCPF {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Error> {
        Self::from_iter(value)
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
        char.is_ascii_digit()
    }

    fn invalid_char_error(invalid_char: InvalidChar) -> Error {
        Error::InvalidChar(invalid_char)
    }

    fn to_output(bytes: [u8; UncheckedCPF::LENGTH]) -> Self::Output {
        UncheckedCPF(bytes)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::cpf;

    use super::*;

    static FORMATTED_STR: &str = "111.444.777";
    static RAW_STR: &str = "111444777";
    pub(crate) static BYTES: [u8; UncheckedCPF::LENGTH] =
        [b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7'];

    #[test]
    fn from_iter() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(
                UncheckedCPF::from_iter(input.chars()),
                Ok(UncheckedCPF(BYTES)),
                "{input}"
            );
        }

        for input in ["111.444.77", "111.444.777-3"] {
            assert_eq!(
                UncheckedCPF::from_iter(input.chars()),
                Err(Error::WrongNumberOfDigits)
            );
        }

        assert_eq!(
            UncheckedCPF::from_iter("111,444.777".chars()),
            Err(Error::InvalidChar(InvalidChar {
                char: ',',
                index: 3
            }))
        );
    }

    #[test]
    fn with_check_digits() {
        let unchecked_cpf = UncheckedCPF(BYTES);

        assert_eq!(unchecked_cpf.with_check_digits(), CPF(cpf::tests::BYTES));
    }

    #[test]
    fn char() {
        let unchecked_cpf = UncheckedCPF(BYTES);

        assert_eq!(unchecked_cpf.char(0), '1');
        assert_eq!(unchecked_cpf.char(1), '1');
        assert_eq!(unchecked_cpf.char(2), '1');
        assert_eq!(unchecked_cpf.char(3), '4');
        assert_eq!(unchecked_cpf.char(4), '4');
        assert_eq!(unchecked_cpf.char(5), '4');
        assert_eq!(unchecked_cpf.char(6), '7');
        assert_eq!(unchecked_cpf.char(7), '7');
        assert_eq!(unchecked_cpf.char(8), '7');
    }

    #[test]
    fn chars() {
        let unchecked_cpf = UncheckedCPF(BYTES);

        assert_eq!(
            unchecked_cpf.chars(),
            ['1', '1', '1', '4', '4', '4', '7', '7', '7']
        );
    }

    #[test]
    fn from_str() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(input.parse(), Ok(UncheckedCPF(BYTES)));
        }
    }

    #[test]
    fn try_from() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(UncheckedCPF::try_from(input), Ok(UncheckedCPF(BYTES)));

            assert_eq!(
                UncheckedCPF::try_from(input.to_string()),
                Ok(UncheckedCPF(BYTES))
            );
        }

        assert_eq!(
            UncheckedCPF::try_from(['1', '1', '1', '4', '4', '4', '7', '7', '7']),
            Ok(UncheckedCPF(BYTES))
        );
    }
}
