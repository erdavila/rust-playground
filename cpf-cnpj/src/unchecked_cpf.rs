use std::{array, fmt::Display, str::FromStr};

use crate::{parser::Parser, Error, InvalidChar, CPF};

unchecked_id!(UncheckedCPF, 9, CPF, UncheckedCPFParser);
impl Display for UncheckedCPF {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.chars();

        write!(
            f,
            "{}{}{}.{}{}{}.{}{}{}",
            chars[0],
            chars[1],
            chars[2],
            chars[3],
            chars[4],
            chars[5],
            chars[6],
            chars[7],
            chars[8],
        )
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

    #[test]
    fn display() {
        let unchecked_cpf = UncheckedCPF(BYTES);

        assert_eq!(unchecked_cpf.to_string(), "111.444.777");
    }
}
