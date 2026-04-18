use crate::comparison::{Scalar, Side};

pub(crate) mod writer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Token {
    Indent,
    Key(String),
    Scalar(Scalar),
    Comma,
    NewLine,
    ArrayBegin,
    ArrayEnd,
    ObjectBegin,
    ObjectEnd,
    BeginMarker(Side),
    EndMarker(Side),
}

impl<T: Into<Scalar>> From<T> for Token {
    fn from(value: T) -> Self {
        Token::Scalar(value.into())
    }
}

pub(crate) trait PutToken {
    type Error;

    fn put_token(&mut self, token: Token) -> Result<(), Self::Error>;
}

impl<T: PutToken> PutToken for &mut T {
    type Error = T::Error;

    fn put_token(&mut self, token: Token) -> Result<(), Self::Error> {
        T::put_token(self, token)
    }
}
