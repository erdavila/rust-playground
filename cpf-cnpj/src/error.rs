use std::fmt::Display;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    WrongNumberOfDigits,
    InvalidChar(InvalidChar),
    InvalidCheckDigitChar(InvalidChar),
    WrongCheckDigits,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::WrongNumberOfDigits => write!(f, "Número errado de dígitos"),
            Error::InvalidChar(invalid_char) => write_invalid_char(f, "dígito", *invalid_char),
            Error::InvalidCheckDigitChar(invalid_char) => {
                write_invalid_char(f, "dígito de verificação", *invalid_char)
            }
            Error::WrongCheckDigits => write!(f, "Dígitos de verificação errados"),
        }
    }
}
impl std::error::Error for Error {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct InvalidChar {
    pub char: char,
    pub index: usize,
}

fn write_invalid_char(
    f: &mut std::fmt::Formatter<'_>,
    ty: &str,
    invalid_char: InvalidChar,
) -> std::fmt::Result {
    write!(
        f,
        "Caractere inválido como {} no índice {}: {:?}",
        ty, invalid_char.index, invalid_char.char
    )
}
