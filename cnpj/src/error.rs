use std::fmt::Display;

#[derive(PartialEq, Eq, Debug)]
pub enum Error {
    WrongNumberOfDigits,
    InvalidChar(InvalidChar),
    InvalidCheckDigitChar(InvalidChar),
    WrongCheckDigits,
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}
impl std::error::Error for Error {}

#[derive(PartialEq, Eq, Debug)]
pub struct InvalidChar {
    pub char: char,
    pub index: usize,
}
