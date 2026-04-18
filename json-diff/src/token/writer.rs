use std::io;

use serde_json::Value;

use super::Token;
use crate::comparison::Side;
use crate::token::PutToken;

pub(crate) struct TokenWriter<T> {
    output: T,
    options: Options,
}
impl<T> TokenWriter<T> {
    pub(crate) fn new(output: T) -> Self {
        Self::new_with_options(output, Options::default())
    }

    pub(crate) fn new_with_options(output: T, options: Options) -> Self {
        Self { output, options }
    }
}
impl<T: io::Write> TokenWriter<T> {
    fn write_byte(&mut self, byte: u8) -> Result<(), io::Error> {
        self.output.write_all(&[byte])
    }
}
impl<T: io::Write> PutToken for TokenWriter<T> {
    type Error = io::Error;

    fn put_token(&mut self, token: Token) -> Result<(), Self::Error> {
        match token {
            Token::Indent => write!(self.output, "{:1$}", "", self.options.indentation_length),
            Token::Key(key) => write!(self.output, "{}: ", Value::from(key)),
            Token::Scalar(scalar) => write!(self.output, "{}", Value::from(scalar)),
            Token::Comma => self.write_byte(b','),
            Token::NewLine => self.write_byte(b'\n'),
            Token::ArrayBegin => self.write_byte(b'['),
            Token::ArrayEnd => self.write_byte(b']'),
            Token::ObjectBegin => self.write_byte(b'{'),
            Token::ObjectEnd => self.write_byte(b'}'),
            Token::BeginMarker(side) => write!(self.output, "{}", self.options.begin_marker(side)),
            Token::EndMarker(side) => write!(self.output, "{}", self.options.end_marker(side)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Options {
    pub(crate) indentation_length: usize,
    pub(crate) left_marker_begin: String,
    pub(crate) left_marker_end: String,
    pub(crate) right_marker_begin: String,
    pub(crate) right_marker_end: String,
}
impl Options {
    fn begin_marker(&self, side: Side) -> &str {
        match side {
            Side::Left => &self.left_marker_begin,
            Side::Right => &self.right_marker_begin,
        }
    }

    fn end_marker(&self, side: Side) -> &str {
        match side {
            Side::Left => &self.left_marker_end,
            Side::Right => &self.right_marker_end,
        }
    }
}
impl Default for Options {
    fn default() -> Self {
        Self {
            indentation_length: 4,
            left_marker_begin: ansi_escape(31 /* red */),
            left_marker_end: ansi_escape(0),
            right_marker_begin: ansi_escape(32 /* green */),
            right_marker_end: ansi_escape(0),
        }
    }
}

fn ansi_escape(code: u8) -> String {
    format!("\x1b[{code}m")
}

#[cfg(test)]
mod tests {

    use super::*;

    fn token_to_string(token: Token) -> String {
        let mut bytes: Vec<u8> = Vec::new();
        let mut writer = TokenWriter::new_with_options(
            &mut bytes,
            Options {
                indentation_length: 3,
                ..Default::default()
            },
        );

        writer.put_token(token).unwrap();

        String::from_utf8(bytes).unwrap()
    }

    #[test]
    fn indent() {
        let output = token_to_string(Token::Indent);

        assert_eq!(output.len(), 3);
        assert!(output.chars().all(|c| c == ' '));
    }

    #[test]
    fn key() {
        let output = token_to_string(Token::Key("abc\x1b".into()));

        assert_eq!(output, r#""abc\u001b": "#);
    }

    #[test]
    fn string() {
        let output = token_to_string(Token::Scalar("abc\x1b".into()));

        assert_eq!(output, r#""abc\u001b""#);
    }
}
