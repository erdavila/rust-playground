use std::{array, fmt::Display, str::FromStr};

use crate::{CNPJ, Error, InvalidChar, parser::Parser};

unchecked_id!(UncheckedCNPJ, 12, CNPJ, UncheckedCNPJParser);
impl Display for UncheckedCNPJ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.chars();

        write!(
            f,
            "{}{}.{}{}{}.{}{}{}/{}{}{}{}",
            chars[0],
            chars[1],
            chars[2],
            chars[3],
            chars[4],
            chars[5],
            chars[6],
            chars[7],
            chars[8],
            chars[9],
            chars[10],
            chars[11],
        )
    }
}

pub(crate) struct UncheckedCNPJParser;
impl Parser<{ UncheckedCNPJ::LENGTH }> for UncheckedCNPJParser {
    type Output = UncheckedCNPJ;

    fn is_digit(char: char) -> bool {
        char.is_ascii_alphanumeric()
    }

    fn invalid_char_error(invalid_char: InvalidChar) -> Error {
        Error::InvalidChar(invalid_char)
    }

    fn to_output(bytes: [u8; UncheckedCNPJ::LENGTH]) -> Self::Output {
        UncheckedCNPJ(bytes)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use crate::InvalidChar;

    use super::*;

    static FORMATTED_STR: &str = "12.AbC.345/01De";
    static RAW_STR: &str = "12AbC34501De";
    pub(crate) static BYTES: [u8; UncheckedCNPJ::LENGTH] = [
        b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E',
    ];

    #[test]
    fn from_iter() {
        for input in [FORMATTED_STR, RAW_STR] {
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

    #[test]
    fn from_str() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(input.parse(), Ok(UncheckedCNPJ(BYTES)));
        }
    }

    #[test]
    fn try_from() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(UncheckedCNPJ::try_from(input), Ok(UncheckedCNPJ(BYTES)));

            assert_eq!(
                UncheckedCNPJ::try_from(input.to_string()),
                Ok(UncheckedCNPJ(BYTES))
            );
        }

        assert_eq!(
            UncheckedCNPJ::try_from(['1', '2', 'A', 'b', 'C', '3', '4', '5', '0', '1', 'D', 'e']),
            Ok(UncheckedCNPJ(BYTES))
        );
    }

    #[test]
    fn display() {
        let unchecked_cnpj = UncheckedCNPJ(BYTES);

        assert_eq!(unchecked_cnpj.to_string(), "12.ABC.345/01DE");
    }
}
