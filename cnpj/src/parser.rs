use crate::{Error, InvalidChar};

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
                } else {
                    Self::ensure_is_formatting_char(char, index)?;
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
            }
            Self::ensure_is_formatting_char(char, index)?;
        }

        Ok(())
    }

    fn ensure_is_formatting_char(char: char, index: usize) -> Result<(), Error> {
        if matches!(char, '.' | '/' | '-') {
            Ok(())
        } else {
            Err(Self::invalid_char_error(InvalidChar { char, index }))
        }
    }
}
