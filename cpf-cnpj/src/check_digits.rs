use std::{array, fmt::Display, str::FromStr};

use crate::{parser::Parser, Error, InvalidChar, UncheckedCNPJ, UncheckedCPF};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CheckDigits(pub(crate) [u8; Self::LENGTH]);
impl CheckDigits {
    pub const LENGTH: usize = 2;

    from_iter!(CheckDigitsParser);
    chars!();

    fn from_unchecked_bytes<const LEN: usize>(
        bytes: [u8; LEN],
        calculator_params: fn(u32) -> (u32, u32),
    ) -> CheckDigits {
        let mut calculators: [_; LEN] = array::from_fn(|i| {
            #[expect(clippy::cast_possible_truncation)]
            let (max_weight, initial_weight) = calculator_params(i as u32);
            CheckDigitCalculator::with_max_and_initial_weight(max_weight, initial_weight)
        });

        for digit in bytes {
            for calculator in &mut calculators {
                calculator.process_digit(digit);
            }
        }

        let mut bytes = [b'\0'; Self::LENGTH];
        for i in 0..Self::LENGTH {
            let check_digit = calculators[i].get_check_digit();
            bytes[i] = check_digit;

            for calculator in &mut calculators[(i + 1)..] {
                calculator.process_digit(check_digit);
            }
        }

        CheckDigits(bytes)
    }
}
impl From<UncheckedCPF> for CheckDigits {
    fn from(unchecked_cpf: UncheckedCPF) -> Self {
        Self::from_unchecked_bytes(unchecked_cpf.0, |i| (11, 10 + i))
    }
}
impl From<UncheckedCNPJ> for CheckDigits {
    fn from(unchecked_cnpj: UncheckedCNPJ) -> Self {
        Self::from_unchecked_bytes(unchecked_cnpj.0, |i| (9, 5 + i))
    }
}
from_str_and_try_from!(CheckDigits);
impl Display for CheckDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let chars = self.chars();

        write!(f, "{}{}", chars[0], chars[1])
    }
}

pub(crate) struct CheckDigitsParser;
impl Parser<{ CheckDigits::LENGTH }> for CheckDigitsParser {
    type Output = CheckDigits;

    fn is_digit(char: char) -> bool {
        char.is_ascii_digit()
    }

    fn invalid_char_error(invalid_char: InvalidChar) -> Error {
        Error::InvalidCheckDigitChar(invalid_char)
    }

    fn to_output(bytes: [u8; CheckDigits::LENGTH]) -> Self::Output {
        CheckDigits(bytes)
    }
}

struct CheckDigitCalculator {
    next_digit_weight: u32,
    accumulator: u32,
    max_weight: u32,
}
impl CheckDigitCalculator {
    fn with_max_and_initial_weight(max_weight: u32, initial_weight: u32) -> Self {
        CheckDigitCalculator {
            next_digit_weight: initial_weight,
            accumulator: 0,
            max_weight,
        }
    }

    fn process_digit(&mut self, digit: u8) {
        let value = match digit {
            b'0'..=b'9' => digit - b'0',
            b'A'..=b'Z' => digit - b'A' + 17,
            _ => unreachable!(),
        };

        self.accumulator += self.next_digit_weight * u32::from(value);
        if self.next_digit_weight == 2 {
            self.next_digit_weight = self.max_weight;
        } else {
            self.next_digit_weight -= 1;
        }
    }

    fn get_check_digit(&self) -> u8 {
        let rem = self.accumulator % 11;
        let value = if rem == 0 || rem == 1 { 0 } else { 11 - rem };

        #[allow(clippy::cast_possible_truncation)]
        let check_digit = value as u8 + b'0';

        check_digit
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) static BYTES: [u8; CheckDigits::LENGTH] = [b'3', b'5'];

    #[test]
    fn from_iter() {
        for input in ["35", "-35", "35."] {
            assert_eq!(
                CheckDigits::from_iter(input.chars()),
                Ok(CheckDigits(BYTES))
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

    #[test]
    fn char() {
        let check_digits = CheckDigits(BYTES);

        assert_eq!(check_digits.char(0), '3');
        assert_eq!(check_digits.char(1), '5');
    }

    #[test]
    fn chars() {
        let check_digits = CheckDigits(BYTES);

        assert_eq!(check_digits.chars(), ['3', '5']);
    }

    #[test]
    fn from_unchecked_cpf() {
        let unchecked_cpf = UncheckedCPF(crate::unchecked_cpf::tests::BYTES);

        assert_eq!(CheckDigits::from(unchecked_cpf), CheckDigits(BYTES));
    }

    #[test]
    fn from_unchecked_cnpj() {
        let unchecked_cnpj = UncheckedCNPJ(crate::unchecked_cnpj::tests::BYTES);

        assert_eq!(CheckDigits::from(unchecked_cnpj), CheckDigits(BYTES));
    }

    #[test]
    fn from_str() {
        assert_eq!("35".parse(), Ok(CheckDigits(BYTES)));
    }

    #[test]
    fn try_from() {
        assert_eq!(CheckDigits::try_from("35"), Ok(CheckDigits(BYTES)));
        assert_eq!(
            CheckDigits::try_from("35".to_string()),
            Ok(CheckDigits(BYTES))
        );
        assert_eq!(CheckDigits::try_from(['3', '5']), Ok(CheckDigits(BYTES)));
    }

    #[test]
    fn display() {
        let check_digits = CheckDigits(BYTES);

        assert_eq!(check_digits.to_string(), "35");
    }
}
