use crate::{unchecked_cnpj::UncheckedCNPJ, CheckDigits, Error, InvalidChar};

pub(crate) trait Parser<const LENGTH: usize> {
    type Output;

    fn is_digit(char: char) -> bool;
    fn invalid_char_error(invalid_char: InvalidChar) -> Error;
    fn to_output(bytes: [u8; LENGTH]) -> Self::Output;

    fn parse(iter: &mut impl Iterator<Item = (usize, char)>) -> Result<Self::Output, Error> {
        let mut count = 0;
        let mut bytes = [b'\0'; LENGTH];

        while count < LENGTH {
            if let Some((index, char)) = iter.next() {
                if Self::is_digit(char) {
                    bytes[count] = char.to_ascii_uppercase() as u8;
                    count += 1;
                } else if !matches!(char, '.' | '/' | '-') {
                    return Err(Self::invalid_char_error(InvalidChar { char, index }));
                }
            } else {
                return Err(Error::WrongNumberOfDigits);
            }
        }

        Ok(Self::to_output(bytes))
    }

    fn ensure_all_consumed(iter: &mut impl Iterator<Item = (usize, char)>) -> Result<(), Error> {
        for (index, char) in iter {
            if Self::is_digit(char) {
                return Err(Error::WrongNumberOfDigits);
            } else if !matches!(char, '.' | '/' | '-') {
                return Err(Self::invalid_char_error(InvalidChar { char, index }));
            }
        }

        Ok(())
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
