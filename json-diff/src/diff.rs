use serde_json::Value;

use crate::comparison::{ArraysComparison, Comparison, ObjectsComparison, Scalar, Side};
use crate::token::{PutToken, Token};

pub(crate) mod inline;
pub(crate) mod line;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sides {
    None,
    LeftOnly,
    RightOnly,
    Both,
}
impl Sides {
    fn from_bools(left: bool, right: bool) -> Self {
        match (left, right) {
            (true, true) => Self::Both,
            (true, false) => Self::LeftOnly,
            (false, true) => Self::RightOnly,
            (false, false) => Self::None,
        }
    }

    fn contains(self, side: Side) -> bool {
        self == Sides::Both || self == Self::only(side)
    }

    fn only(side: Side) -> Sides {
        match side {
            Side::Left => Sides::LeftOnly,
            Side::Right => Sides::RightOnly,
        }
    }
}
impl From<Side> for Sides {
    fn from(value: Side) -> Self {
        match value {
            Side::Left => Self::LeftOnly,
            Side::Right => Self::RightOnly,
        }
    }
}

trait Diff<T: PutToken> {
    fn output(&mut self) -> &mut T;

    fn put_prefix_tokens(&mut self, indent: usize, key: Option<String>) -> Result<(), T::Error> {
        self.put_indent_tokens(indent)?;

        if let Some(key) = key {
            self.output().put_token(Token::Key(key))?;
        }

        Ok(())
    }

    fn put_indent_tokens(&mut self, indent: usize) -> Result<(), T::Error> {
        for _ in 0..indent {
            self.output().put_token(Token::Indent)?;
        }

        Ok(())
    }

    fn put_value_tokens(
        &mut self,
        indent: usize,
        value: Value,
        comma: bool,
    ) -> Result<(), T::Error> {
        match value {
            Value::Null => self.output().put_token(Scalar::Null.into())?,
            Value::Bool(bool) => self.output().put_token(bool.into())?,
            Value::Number(number) => self.output().put_token(number.into())?,
            Value::String(string) => self.output().put_token(string.into())?,
            Value::Array(values) => {
                let container = Container {
                    type_: ContainerType::Array,
                    length: values.len(),
                    entries: values.into_iter().map(|value| (None, value)),
                };
                self.put_container_tokens(indent, container)?;
            }
            Value::Object(map) => {
                let mut entries: Vec<_> = map
                    .into_iter()
                    .map(|(key, value)| (Some(key), value))
                    .collect();
                entries.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

                let container = Container {
                    type_: ContainerType::Object,
                    length: entries.len(),
                    entries,
                };
                self.put_container_tokens(indent, container)?;
            }
        }

        if comma {
            self.output().put_token(Token::Comma)?;
        }

        Ok(())
    }

    fn put_container_tokens<I>(
        &mut self,
        indent: usize,
        container: Container<I>,
    ) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = (Option<String>, Value)>,
    {
        if container.length == 0 {
            self.put_empty_container_tokens(container.type_)
        } else {
            self.output().put_token(container.type_.begin_token())?;
            self.output().put_token(Token::NewLine)?;

            let mut count = container.length;
            for (key, value) in container.entries {
                count -= 1;
                self.put_prefix_tokens(indent + 1, key)?;
                self.put_value_tokens(indent + 1, value, count > 0)?;
                self.output().put_token(Token::NewLine)?;
            }

            self.put_indent_tokens(indent)?;
            self.output().put_token(container.type_.end_token())
        }
    }

    fn put_empty_container_tokens(&mut self, type_: ContainerType) -> Result<(), T::Error> {
        self.output().put_token(type_.begin_token())?;
        self.output().put_token(type_.end_token())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ContainerType {
    Array,
    Object,
}
impl ContainerType {
    fn begin_token(self) -> Token {
        match self {
            ContainerType::Array => Token::ArrayBegin,
            ContainerType::Object => Token::ObjectBegin,
        }
    }

    fn end_token(self) -> Token {
        match self {
            ContainerType::Array => Token::ArrayEnd,
            ContainerType::Object => Token::ObjectEnd,
        }
    }
}

struct Container<I> {
    type_: ContainerType,
    length: usize,
    entries: I,
}

struct ContainersComparison<I> {
    type_: ContainerType,
    left_length: usize,
    right_length: usize,
    entries: I,
}
impl<I> ContainersComparison<I>
where
    I: IntoIterator<Item = ComparisonEntry>,
{
    fn into_one_side(self, side: Side) -> Container<impl Iterator<Item = (Option<String>, Value)>> {
        let length = match side {
            Side::Left => self.left_length,
            Side::Right => self.right_length,
        };

        Container {
            type_: self.type_,
            length,
            entries: self.entries.into_iter().map(|(key, value)| {
                let ComparisonEntryValue::OneSideOnly(value, _) = value else {
                    unreachable!()
                };
                (key, value)
            }),
        }
    }
}

type ComparisonEntry = (Option<String>, ComparisonEntryValue);

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComparisonEntryValue {
    Comparison(Comparison),
    OneSideOnly(Value, Side),
}

trait IntoContainersComparison {
    fn into_containers_comparison(
        self,
    ) -> ContainersComparison<impl IntoIterator<Item = ComparisonEntry>>;
}
impl IntoContainersComparison for ArraysComparison {
    fn into_containers_comparison(
        self,
    ) -> ContainersComparison<impl IntoIterator<Item = ComparisonEntry>> {
        let mut left_length = self.common_indexes.len();
        let mut right_length = self.common_indexes.len();
        if let Some(sided_entries) = &self.one_side_only_indexes {
            match sided_entries.side {
                Side::Left => left_length += sided_entries.values.len(),
                Side::Right => right_length += sided_entries.values.len(),
            }
        }

        let common_entries = self
            .common_indexes
            .into_iter()
            .map(|cmp| (None, ComparisonEntryValue::Comparison(cmp)));

        let one_side_entries = self.one_side_only_indexes.into_iter().flat_map(|sided| {
            sided
                .values
                .into_iter()
                .map(move |value| (None, ComparisonEntryValue::OneSideOnly(value, sided.side)))
        });

        ContainersComparison {
            type_: ContainerType::Array,
            left_length,
            right_length,
            entries: common_entries.chain(one_side_entries),
        }
    }
}
impl IntoContainersComparison for ObjectsComparison {
    fn into_containers_comparison(
        self,
    ) -> ContainersComparison<impl IntoIterator<Item = ComparisonEntry>> {
        let left_length = self.common_entries.len() + self.left_only_entries.len();
        let right_length = self.common_entries.len() + self.right_only_entries.len();

        let common_entries = self
            .common_entries
            .into_iter()
            .map(|(key, cmp)| (Some(key), ComparisonEntryValue::Comparison(cmp)));
        let [left_only_entries, right_only_entries] = [
            (self.left_only_entries, Side::Left),
            (self.right_only_entries, Side::Right),
        ]
        .map(|(map, side)| {
            map.into_iter().map(move |(key, value)| {
                (Some(key), ComparisonEntryValue::OneSideOnly(value, side))
            })
        });

        let mut entries: Vec<_> = common_entries
            .chain(left_only_entries)
            .chain(right_only_entries)
            .collect();
        entries.sort_by(|(key1, _), (key2, _)| key1.cmp(key2));

        ContainersComparison {
            type_: ContainerType::Object,
            left_length,
            right_length,
            entries,
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use std::convert::Infallible;

    use crate::token::{PutToken, Token};

    macro_rules! tokens {
        ($($tt:tt)*) => {
            $crate::diff::tests::__tokens!([] $($tt)*)
        };
    }
    pub(crate) use tokens;

    macro_rules! __tokens {
        (
            [ $( $tk:expr, )* ]
            Null
            $( , $( $tt:tt )* )?
        ) => {
            $crate::diff::tests::__tokens!(
                [
                    $( $tk, )*
                    Token::Scalar(Scalar::Null),
                ]
                $( $( $tt )* )?
            )
        };

        (
            [ $( $tk:expr, )* ]
            $name:ident
            $( , $( $tt:tt )* )?
        ) => {
            $crate::diff::tests::__tokens!(
                [
                    $( $tk, )*
                    Token::$name,
                ]
                $( $( $tt )* )?
            )
        };

        (
            [ $( $tk:expr, )* ]
            $name:ident ( $arg:expr )
            $( , $( $tt:tt )* )?
        ) => {
            $crate::diff::tests::__tokens!(
                [
                    $( $tk, )*
                    Token::$name(($arg).into()),
                ]
                $( $( $tt )* )?
            )
        };

        (
            [ $( $tk:expr, )* ]
            $side:ident { $( $name:ident $( ( $arg:expr ) )? ),* $(,)? }
            $( , $( $tt:tt )* )?
        ) => {
            $crate::diff::tests::__tokens!(
                [
                    $( $tk, )*
                ]
                BeginMarker(Side::$side),
                $( $name $( ( $arg ) )?, )*
                EndMarker(Side::$side),
                $( $( $tt )* )?
            )
        };

        (
            [ $( $tk:expr, )* ]
        ) => {
            [ $( $tk ),* ].to_vec()
        };

    }
    pub(crate) use __tokens;

    pub(crate) struct TestPutToken {
        pub(crate) tokens: Vec<Token>,
    }
    impl TestPutToken {
        pub(crate) fn new() -> Self {
            Self { tokens: Vec::new() }
        }
    }
    impl PutToken for TestPutToken {
        type Error = Infallible;

        fn put_token(&mut self, token: Token) -> Result<(), Self::Error> {
            self.tokens.push(token);
            Ok(())
        }
    }
}
