use std::convert::Infallible;
use std::fs;

use anyhow::{Result, bail};
use serde_json::{Number, Value, from_str};

fn main() -> Result<()> {
    let mut left_file = None;
    let mut right_file = None;

    for arg in std::env::args().skip(1) {
        match arg.as_str() {
            "-h" | "--help" => {
                println!("Usage: json-diff <LEFT_FILE> <RIGHT_FILE>");
                return Ok(());
            }
            _ => {
                if left_file.is_none() {
                    left_file = Some(arg);
                } else if right_file.is_none() {
                    right_file = Some(arg);
                } else {
                    bail!("Too many arguments");
                }
            }
        }
    }

    let (Some(left_file), Some(right_file)) = (left_file, right_file) else {
        bail!("Not enough arguments");
    };

    let left_value = from_str(&fs::read_to_string(left_file)?)?;
    let right_value = from_str(&fs::read_to_string(right_file)?)?;

    let mut comparator = Comparator::new(Writer);
    comparator.compare(left_value, right_value)?;

    Ok(())
}

struct Writer;
impl Write for Writer {
    type Error = Infallible;

    fn write(&mut self, chunk: Chunk) -> Result<(), Self::Error> {
        match chunk {
            Chunk::Indent(n) => todo!(),
            Chunk::Position(pos) => todo!(),
            Chunk::Comma => todo!(),
            Chunk::NewLine => todo!(),
            Chunk::Null => todo!(),
            Chunk::Bool(_) => todo!(),
            Chunk::Number(number) => todo!(),
            Chunk::String(_) => todo!(),
            Chunk::ArrayBegin => todo!(),
            Chunk::ArrayEnd => todo!(),
            Chunk::Left(chunk) => todo!(),
            Chunk::Right(chunk) => todo!(),
            Chunk::Changed(chunk) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Chunk {
    Indent(usize),
    Position(Position),
    Comma,
    NewLine,
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    ArrayBegin,
    ArrayEnd,
    Left(Box<Chunk>),
    Right(Box<Chunk>),
    Changed(Box<Chunk>),
}
impl Chunk {
    fn left(chunk: Chunk) -> Self {
        Self::Left(Box::new(chunk))
    }

    fn right(chunk: Chunk) -> Self {
        Self::Right(Box::new(chunk))
    }

    fn changed(chunk: Chunk) -> Self {
        Self::Changed(Box::new(chunk))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum Position {
    Index(usize),
    Key(String),
}

trait Write {
    type Error;

    fn write(&mut self, chunk: Chunk) -> Result<(), Self::Error>;
}

impl<W: Write> Write for &mut W {
    type Error = W::Error;

    fn write(&mut self, chunk: Chunk) -> Result<(), Self::Error> {
        W::write(self, chunk)
    }
}

struct Comparator<W> {
    writer: W,
    indent: usize,
}
impl<W> Comparator<W> {
    fn new(writer: W) -> Self {
        Self { writer, indent: 0 }
    }
}
impl<W: Write> Comparator<W> {
    fn compare(&mut self, left: Value, right: Value) -> Result<(), W::Error> {
        self.compare_values(None, left, right, false, false)
    }

    fn compare_values(
        &mut self,
        position: Option<Position>,
        left: Value,
        right: Value,
        left_comma: bool,
        right_comma: bool,
    ) -> Result<(), W::Error> {
        match (left, right) {
            (Value::Null, Value::Null) => {
                self.write_unchanged_scalar(
                    Value::Null,
                    position,
                    |_| Chunk::Null,
                    left_comma,
                    right_comma,
                )?;
            }
            (Value::Null, Value::Bool(_)) => todo!(),
            (Value::Null, Value::Number(number)) => todo!(),
            (Value::Null, Value::String(_)) => todo!(),
            (Value::Null, Value::Array(values)) => todo!(),
            (Value::Null, Value::Object(map)) => todo!(),
            (Value::Bool(_), Value::Null) => todo!(),
            (Value::Bool(left), Value::Bool(right)) => {
                self.compare_scalars(left, right, position, Chunk::Bool, left_comma, right_comma)?;
            }
            (Value::Bool(_), Value::Number(number)) => todo!(),
            (Value::Bool(_), Value::String(_)) => todo!(),
            (Value::Bool(_), Value::Array(values)) => todo!(),
            (Value::Bool(_), Value::Object(map)) => todo!(),
            (Value::Number(number), Value::Null) => todo!(),
            (Value::Number(number), Value::Bool(_)) => todo!(),
            (Value::Number(left), Value::Number(right)) => {
                self.compare_scalars(
                    left,
                    right,
                    position,
                    Chunk::Number,
                    left_comma,
                    right_comma,
                )?;
            }
            (Value::Number(number), Value::String(_)) => todo!(),
            (Value::Number(number), Value::Array(values)) => todo!(),
            (Value::Number(number), Value::Object(map)) => todo!(),
            (Value::String(_), Value::Null) => todo!(),
            (Value::String(_), Value::Bool(_)) => todo!(),
            (Value::String(_), Value::Number(number)) => todo!(),
            (Value::String(left), Value::String(right)) => {
                self.compare_scalars(
                    left,
                    right,
                    position,
                    Chunk::String,
                    left_comma,
                    right_comma,
                )?;
            }
            (Value::String(_), Value::Array(values)) => todo!(),
            (Value::String(_), Value::Object(map)) => todo!(),
            (Value::Array(values), Value::Null) => todo!(),
            (Value::Array(values), Value::Bool(_)) => todo!(),
            (Value::Array(values), Value::Number(number)) => todo!(),
            (Value::Array(values), Value::String(_)) => todo!(),
            (Value::Array(left), Value::Array(right)) => self.compare_arrays(left, right)?,
            (Value::Array(values), Value::Object(map)) => todo!(),
            (Value::Object(map), Value::Null) => todo!(),
            (Value::Object(map), Value::Bool(_)) => todo!(),
            (Value::Object(map), Value::Number(number)) => todo!(),
            (Value::Object(map), Value::String(_)) => todo!(),
            (Value::Object(map), Value::Array(values)) => todo!(),
            (Value::Object(map), Value::Object(_)) => todo!(),
        }

        Ok(())
    }

    fn compare_scalars<T: PartialEq>(
        &mut self,
        left: T,
        right: T,
        position: Option<Position>,
        to_chunk: fn(T) -> Chunk,
        left_comma: bool,
        right_comma: bool,
    ) -> Result<(), W::Error> {
        if left == right {
            self.write_unchanged_scalar(left, position, to_chunk, left_comma, right_comma)?;
        } else {
            self.write_changed_scalar(left, position.as_ref(), to_chunk, Chunk::left, left_comma)?;
            self.write_changed_scalar(
                right,
                position.as_ref(),
                to_chunk,
                Chunk::right,
                right_comma,
            )?;
        }
        Ok(())
    }

    fn compare_arrays(&mut self, left: Vec<Value>, right: Vec<Value>) -> Result<(), W::Error> {
        match (left.is_empty(), right.is_empty()) {
            (true, true) => {
                self.write_indent()?;
                self.write(Chunk::ArrayBegin)?;
                self.write(Chunk::ArrayEnd)?;
                self.write_newline()?;
            }
            (true, false) => todo!(),
            (false, true) => todo!(),
            (false, false) => {
                self.write_indent()?;
                self.write(Chunk::ArrayBegin)?;
                self.write_newline()?;

                self.indent += 1;

                let mut index = 0;

                let left_len = left.len();
                let right_len = right.len();

                let mut left = left.into_iter();
                let mut right = right.into_iter();

                loop {
                    match (left.next(), right.next()) {
                        (Some(left), Some(right)) => {
                            let left_comma = index < left_len - 1;
                            let right_comma = index < right_len - 1;
                            self.compare_values(
                                Some(Position::Index(index)),
                                left,
                                right,
                                left_comma,
                                right_comma,
                            )?;
                        }
                        (Some(left), None) => match value_to_scalar_chunk(left) {
                            Ok(chunk) => {
                                let left_comma = index < left_len - 1;
                                self.write_one_side_only_scalar(
                                    chunk,
                                    Position::Index(index),
                                    Chunk::left,
                                    left_comma,
                                )?;
                            }
                            Err(_) => todo!(),
                        },
                        (None, Some(right)) => match value_to_scalar_chunk(right) {
                            Ok(chunk) => {
                                let right_comma = index < right_len - 1;
                                self.write_one_side_only_scalar(
                                    chunk,
                                    Position::Index(index),
                                    Chunk::right,
                                    right_comma,
                                )?;
                            }
                            Err(value) => todo!(),
                        },
                        (None, None) => break,
                    }

                    index += 1;
                }

                self.indent -= 1;

                self.write_indent()?;
                self.write(Chunk::ArrayEnd)?;
                // TODO: comma?
                self.write_newline()?;
            }
        }

        Ok(())
    }

    fn write(&mut self, chunk: Chunk) -> Result<(), W::Error> where {
        self.writer.write(chunk)
    }

    fn write_unchanged_scalar<T>(
        &mut self,
        scalar: T,
        position: Option<Position>,
        to_chunk: fn(T) -> Chunk,
        left_comma: bool,
        right_comma: bool,
    ) -> Result<(), W::Error> {
        self.write_indent()?;
        if let Some(pos) = position {
            self.write(Chunk::Position(pos))?;
        }
        self.write(to_chunk(scalar))?;
        match (left_comma, right_comma) {
            (true, true) => self.write(Chunk::Comma)?,
            (true, false) => self.write(Chunk::left(Chunk::Comma))?,
            (false, true) => self.write(Chunk::right(Chunk::Comma))?,
            (false, false) => {}
        }
        self.write_newline()?;
        Ok(())
    }

    fn write_changed_scalar<T>(
        &mut self,
        scalar: T,
        position: Option<&Position>,
        to_chunk: fn(T) -> Chunk,
        to_side: fn(Chunk) -> Chunk,
        comma: bool,
    ) -> Result<(), W::Error> {
        self.write_indent()?;
        if let Some(pos) = position {
            self.write(Chunk::changed(Chunk::Position(pos.clone())))?;
        }
        self.write(to_side(to_chunk(scalar)))?;
        if comma {
            self.write(to_side(Chunk::Comma))?;
        }
        self.write_newline()?;
        Ok(())
    }

    fn write_one_side_only_scalar(
        &mut self,
        scalar_chunk: Chunk,
        position: Position,
        to_side: fn(Chunk) -> Chunk,
        comma: bool,
    ) -> Result<(), W::Error> {
        self.write_indent()?;
        self.write(to_side(Chunk::Position(position)))?;
        self.write(to_side(scalar_chunk))?;
        if comma {
            self.write(to_side(Chunk::Comma))?;
        }
        self.write_newline()?;
        Ok(())
    }

    fn write_indent(&mut self) -> Result<(), W::Error> {
        if self.indent > 0 {
            self.write(Chunk::Indent(self.indent))?;
        }
        Ok(())
    }

    fn write_newline(&mut self) -> Result<(), W::Error> {
        self.write(Chunk::NewLine)
    }
}

fn value_to_scalar_chunk(value: Value) -> Result<Chunk, Value> {
    match value {
        Value::Null => todo!(),
        Value::Bool(bool) => Ok(Chunk::Bool(bool)),
        Value::Number(number) => Ok(Chunk::Number(number)),
        Value::String(string) => Ok(Chunk::String(string)),
        Value::Array(values) => todo!(),
        Value::Object(map) => todo!(),
    }
}

#[cfg(test)]
mod tests;
