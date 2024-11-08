use std::{fmt::Display, str::FromStr};

use crate::{UncheckedCPF, UncheckedCPFParser};

checked_id!(CPF, UncheckedCPF, UncheckedCPFParser);
from_str_and_try_from!(CPF);

#[cfg(test)]
pub(crate) mod tests {
    use crate::{check_digits, unchecked_cpf, CheckDigits, Error, InvalidChar};

    use super::*;

    static FORMATTED_STR: &str = "111.444.777-35";
    static RAW_STR: &str = "11144477735";
    pub(crate) static BYTES: [u8; CPF::LENGTH] = [
        b'1', b'1', b'1', b'4', b'4', b'4', b'7', b'7', b'7', b'3', b'5',
    ];
    static CHARS: [char; CPF::LENGTH] = ['1', '1', '1', '4', '4', '4', '7', '7', '7', '3', '5'];

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

    #[test]
    fn char() {
        let cpf = CPF(BYTES);

        assert_eq!(cpf.char(0), '1');
        assert_eq!(cpf.char(1), '1');
        assert_eq!(cpf.char(2), '1');
        assert_eq!(cpf.char(3), '4');
        assert_eq!(cpf.char(4), '4');
        assert_eq!(cpf.char(5), '4');
        assert_eq!(cpf.char(6), '7');
        assert_eq!(cpf.char(7), '7');
        assert_eq!(cpf.char(8), '7');
        assert_eq!(cpf.char(9), '3');
        assert_eq!(cpf.char(10), '5');
    }

    #[test]
    fn chars() {
        let cpf = CPF(BYTES);

        assert_eq!(cpf.chars(), CHARS);
    }

    #[test]
    fn from_str() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(input.parse(), Ok(CPF(BYTES)));
        }
    }

    #[test]
    fn try_from() {
        for input in [FORMATTED_STR, RAW_STR] {
            assert_eq!(CPF::try_from(input), Ok(CPF(BYTES)));

            assert_eq!(CPF::try_from(input.to_string()), Ok(CPF(BYTES)));
        }

        assert_eq!(CPF::try_from(CHARS), Ok(CPF(BYTES)));
    }

    #[test]
    fn display() {
        let cpf = CPF(BYTES);

        assert_eq!(cpf.to_string(), FORMATTED_STR);
    }
}
