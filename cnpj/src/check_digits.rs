use std::{array, fmt::Display};

use crate::{Error, InvalidChar, UncheckedCNPJ};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct CheckDigits([u8; Self::LENGTH]);
impl CheckDigits {
    pub const LENGTH: usize = 2;

    fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
        let mut count = 0;
        let mut bytes = [b'\0'; Self::LENGTH];

        for (index, char) in iter.into_iter().enumerate() {
            if char.is_ascii_digit() {
                if count < Self::LENGTH {
                    bytes[count] = char as u8;
                    count += 1;
                } else {
                    return Err(Error::WrongNumberOfDigits);
                }
            } else if !matches!(char, '.' | '/' | '-') {
                return Err(Error::InvalidCheckDigitChar(InvalidChar { char, index }));
            }
        }

        if count != Self::LENGTH {
            return Err(Error::WrongNumberOfDigits);
        }

        Ok(CheckDigits(bytes))
    }

    #[must_use]
    pub fn char(self, index: usize) -> char {
        self.0[index].into()
    }

    pub fn chars(self) -> [char; Self::LENGTH] {
        self.0.map(Into::into)
    }
}
impl From<UncheckedCNPJ> for CheckDigits {
    fn from(unchecked_cnpj: UncheckedCNPJ) -> Self {
        #[expect(clippy::cast_possible_truncation)]
        let mut calculators: [_; Self::LENGTH] =
            array::from_fn(|i| CheckDigitCalculator::with_initial_weight(5 + i as u32));

        for digit in unchecked_cnpj.0 {
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
impl TryFrom<&str> for CheckDigits {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<String> for CheckDigits {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl TryFrom<[char; Self::LENGTH]> for CheckDigits {
    type Error = Error;

    fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Self::Error> {
        todo!()
    }
}
impl Display for CheckDigits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

struct CheckDigitCalculator {
    next_digit_weight: u32,
    accumulator: u32,
}
impl CheckDigitCalculator {
    fn with_initial_weight(weight: u32) -> Self {
        CheckDigitCalculator {
            next_digit_weight: weight,
            accumulator: 0,
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
            self.next_digit_weight = 9;
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
mod tests {
    use super::*;

    static BYTES: [u8; CheckDigits::LENGTH] = [b'3', b'5'];

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
    fn from() {
        let unchecked_cnpj = UncheckedCNPJ([
            b'1', b'2', b'A', b'B', b'C', b'3', b'4', b'5', b'0', b'1', b'D', b'E',
        ]);

        assert_eq!(CheckDigits::from(unchecked_cnpj), CheckDigits(BYTES));
    }
}
