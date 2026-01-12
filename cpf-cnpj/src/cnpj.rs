use std::{fmt::Display, str::FromStr};

use crate::{UncheckedCNPJ, UncheckedCNPJParser};

checked_id!(CNPJ, UncheckedCNPJ, UncheckedCNPJParser);
from_str_and_try_from!(CNPJ);

#[cfg(test)]
mod tests {
    use crate::{CheckDigits, Error, InvalidChar, check_digits, unchecked_cnpj};

    use super::*;

    static FORMATTED_STR: &str = "12.AbC.345/01De-35";
    static RAW_STR: &str = "12ABC34501DE35";
    static BYTES: [u8; CNPJ::LENGTH] = [
        b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E', b'3', b'5',
    ];

    #[test]
    fn from_iter() {
        for input in [FORMATTED_STR, RAW_STR] {
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
            [
                '1', '2', 'A', 'B', 'C', '3', '4', '5', '0', '1', 'D', 'E', '3', '5'
            ]
        );
    }

    #[test]
    fn from_str() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(input.parse(), Ok(CNPJ(BYTES)));
        }
    }

    #[test]
    fn try_from() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(CNPJ::try_from(input), Ok(CNPJ(BYTES)));

            assert_eq!(CNPJ::try_from(input.to_string()), Ok(CNPJ(BYTES)));
        }

        assert_eq!(
            CNPJ::try_from([
                '1', '2', 'A', 'b', 'C', '3', '4', '5', '0', '1', 'D', 'e', '3', '5'
            ]),
            Ok(CNPJ(BYTES))
        );
    }

    #[test]
    fn display() {
        let cnpj = CNPJ(BYTES);

        assert_eq!(cnpj.to_string(), "12.ABC.345/01DE-35");
    }
}
