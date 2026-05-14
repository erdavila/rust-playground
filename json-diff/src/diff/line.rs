use std::collections::BTreeMap;

use serde_json::Value;

use crate::comparison::{
    ArraysComparison, Comparison, ObjectsComparison, Scalar, ScalarPair, ScalarsComparison, Side,
};
use crate::token::{PutToken, Token};

pub(crate) fn tokenize<T: PutToken>(output: T, comparison: Comparison) -> Result<(), T::Error> {
    let mut line_diff = LineDiff { output };
    line_diff.put_comparison_tokens(0, None, comparison, Sides::None)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sides {
    None,
    LeftOnly,
    RightOnly,
    Both,
}
impl Sides {
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

struct LineDiff<T> {
    output: T,
}
impl<T: PutToken> LineDiff<T> {
    fn put_comparison_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        comparison: Comparison,
        commas: Sides,
    ) -> Result<(), T::Error> {
        match comparison {
            Comparison::Scalars(ScalarsComparison::Same(scalar)) => {
                self.by_commas(commas, |this, comma| {
                    this.put_line_tokens(
                        indent,
                        key.clone(),
                        [Token::Scalar(scalar.clone())],
                        comma,
                    )
                })
            }
            Comparison::Scalars(ScalarsComparison::Different(scalar_pair)) => {
                let (left, right) = match scalar_pair {
                    ScalarPair::Bools(left, right) => (left.into(), right.into()),
                    ScalarPair::Numbers(left, right) => (left.into(), right.into()),
                    ScalarPair::Strings(left, right) => (left.into(), right.into()),
                };

                self.put_both_sides_tokens(left, right, |this, value, side| {
                    this.put_value_tokens(indent, key.clone(), value, commas.contains(side))
                })
            }
            Comparison::Arrays(ArraysComparison {
                common_indexes,
                one_side_only_indexes,
            }) => {
                let mut left_length = common_indexes.len();
                let mut right_length = common_indexes.len();
                if let Some(sided_entries) = &one_side_only_indexes {
                    match sided_entries.side {
                        Side::Left => left_length += sided_entries.values.len(),
                        Side::Right => right_length += sided_entries.values.len(),
                    }
                }

                let common_entries = common_indexes
                    .into_iter()
                    .map(|cmp| (None, ComparisonEntryValue::Comparison(cmp)));

                let one_side_entries = one_side_only_indexes.into_iter().flat_map(|sided| {
                    sided.values.into_iter().map(move |value| {
                        (None, ComparisonEntryValue::OneSideOnly(value, sided.side))
                    })
                });

                let containers_cmp = ContainersComparison {
                    type_: ContainerType::Array,
                    left_length,
                    right_length,
                    entries: common_entries.chain(one_side_entries),
                };

                self.put_containers_comparison_tokens(indent, key, containers_cmp, commas)
            }
            Comparison::Objects(ObjectsComparison {
                common_entries,
                left_only_entries,
                right_only_entries,
            }) => {
                let left_length = common_entries.len() + left_only_entries.len();
                let right_length = common_entries.len() + right_only_entries.len();

                let common_entries = common_entries
                    .into_iter()
                    .map(|(key, cmp)| (Some(key), ComparisonEntryValue::Comparison(cmp)));
                let [left_only_entries, right_only_entries] = [
                    (left_only_entries, Side::Left),
                    (right_only_entries, Side::Right),
                ]
                .map(|(map, side)| {
                    map.into_iter().map(move |(key, value)| {
                        (Some(key), ComparisonEntryValue::OneSideOnly(value, side))
                    })
                });

                // A BTreeMap ensures that the entries are sorted
                let entries: BTreeMap<_, _> = common_entries
                    .chain(left_only_entries)
                    .chain(right_only_entries)
                    .collect();

                let containers_cmp = ContainersComparison {
                    type_: ContainerType::Object,
                    left_length,
                    right_length,
                    entries,
                };

                self.put_containers_comparison_tokens(indent, key, containers_cmp, commas)
            }
            Comparison::DifferentTypes(left, right) => {
                self.put_both_sides_tokens(left, right, |this, value, side| {
                    this.put_value_tokens(indent, key.clone(), value, commas.contains(side))
                })
            }
        }
    }

    fn put_value_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        value: Value,
        comma: bool,
    ) -> Result<(), T::Error> {
        match value {
            Value::Null => self.put_line_tokens(indent, key, [Scalar::Null.into()], comma),
            Value::Bool(bool) => self.put_line_tokens(indent, key, [bool.into()], comma),
            Value::Number(number) => self.put_line_tokens(indent, key, [number.into()], comma),
            Value::String(string) => self.put_line_tokens(indent, key, [string.into()], comma),
            Value::Array(values) => {
                let container = Container {
                    type_: ContainerType::Array,
                    length: values.len(),
                    entries: values.into_iter().map(|value| (None, value)),
                };
                self.put_container_tokens(indent, key, container, comma)
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
                self.put_container_tokens(indent, key, container, comma)
            }
        }
    }

    fn put_container_tokens<I>(
        &mut self,
        indent: usize,
        key: Option<String>,
        container: Container<I>,
        comma: bool,
    ) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = (Option<String>, Value)>,
    {
        if container.length == 0 {
            self.put_empty_container_tokens(container.type_, indent, key, comma)
        } else {
            self.put_line_tokens(indent, key, [container.type_.begin_token()], false)?;

            let mut count = container.length;
            for (key, value) in container.entries {
                count -= 1;
                self.put_value_tokens(indent + 1, key, value, count > 0)?;
            }

            self.put_line_tokens(indent, None, [container.type_.end_token()], comma)
        }
    }

    fn put_containers_comparison_tokens<I>(
        &mut self,
        indent: usize,
        key: Option<String>,
        containers_cmp: ContainersComparison<I>,
        commas: Sides,
    ) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = ComparisonEntry>,
    {
        let type_ = containers_cmp.type_;

        match (
            containers_cmp.left_length == 0,
            containers_cmp.right_length == 0,
        ) {
            (true, true) => self.by_commas(commas, |this, comma| {
                this.put_empty_container_tokens(type_, indent, key.clone(), comma)
            }),
            (true, false) => {
                self.put_sided_tokens(Side::Left, |this| {
                    this.put_empty_container_tokens(type_, indent, key, commas.contains(Side::Left))
                })?;

                self.put_sided_tokens(Side::Right, |this| {
                    let container = containers_cmp.into_one_side(Side::Right);
                    this.put_container_tokens(indent, None, container, commas.contains(Side::Right))
                })
            }
            (false, true) => {
                self.put_sided_tokens(Side::Left, |this| {
                    let container = containers_cmp.into_one_side(Side::Left);
                    this.put_container_tokens(indent, None, container, commas.contains(Side::Left))
                })?;

                self.put_sided_tokens(Side::Right, |this| {
                    this.put_empty_container_tokens(
                        type_,
                        indent,
                        key,
                        commas.contains(Side::Right),
                    )
                })
            }
            (false, false) => {
                self.put_line_tokens(indent, key, [containers_cmp.type_.begin_token()], false)?;

                let mut left_count = containers_cmp.left_length;
                let mut right_count = containers_cmp.right_length;

                for (key, value) in containers_cmp.entries {
                    match value {
                        ComparisonEntryValue::Comparison(comparison) => {
                            left_count -= 1;
                            right_count -= 1;
                            let commas = match (left_count > 0, right_count > 0) {
                                (true, true) => Sides::Both,
                                (true, false) => Sides::LeftOnly,
                                (false, true) => Sides::RightOnly,
                                (false, false) => Sides::None,
                            };
                            self.put_comparison_tokens(indent + 1, key, comparison, commas)?;
                        }
                        ComparisonEntryValue::OneSideOnly(value, side) => {
                            let count = match side {
                                Side::Left => &mut left_count,
                                Side::Right => &mut right_count,
                            };
                            *count -= 1;

                            self.put_sided_tokens(side, |this| {
                                this.put_value_tokens(indent + 1, key, value, *count > 0)
                            })?;
                        }
                    }
                }

                self.by_commas(commas, |this, comma| {
                    this.put_line_tokens(indent, None, [type_.end_token()], comma)
                })
            }
        }
    }

    fn put_empty_container_tokens(
        &mut self,
        type_: ContainerType,
        indent: usize,
        key: Option<String>,
        comma: bool,
    ) -> Result<(), T::Error> {
        self.put_line_tokens(indent, key, [type_.begin_token(), type_.end_token()], comma)
    }

    fn put_line_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        content: impl IntoIterator<Item = Token>,
        comma: bool,
    ) -> Result<(), T::Error> {
        for _ in 0..indent {
            self.output.put_token(Token::Indent)?;
        }

        if let Some(key) = key {
            self.output.put_token(Token::Key(key))?;
        }

        for token in content {
            self.output.put_token(token)?;
        }

        if comma {
            self.output.put_token(Token::Comma)?;
        }

        self.output.put_token(Token::NewLine)
    }

    fn put_both_sides_tokens<U>(
        &mut self,
        left: U,
        right: U,
        mut f: impl FnMut(&mut Self, U, Side) -> Result<(), T::Error>,
    ) -> Result<(), T::Error> {
        for (value, side) in [(left, Side::Left), (right, Side::Right)] {
            self.put_sided_tokens(side, |this| f(this, value, side))?;
        }
        Ok(())
    }

    fn put_sided_tokens(
        &mut self,
        side: Side,
        f: impl FnOnce(&mut Self) -> Result<(), T::Error>,
    ) -> Result<(), T::Error> {
        self.output.put_token(Token::BeginMarker(side))?;
        f(self)?;
        self.output.put_token(Token::EndMarker(side))
    }

    fn by_commas(
        &mut self,
        commas: Sides,
        mut f: impl FnMut(&mut Self, bool) -> Result<(), T::Error>,
    ) -> Result<(), T::Error> {
        match commas {
            Sides::None => f(self, false),
            Sides::LeftOnly => {
                self.put_both_sides_tokens(true, false, |this, comma, _side| f(this, comma))
            }
            Sides::RightOnly => {
                self.put_both_sides_tokens(false, true, |this, comma, _side| f(this, comma))
            }
            Sides::Both => f(self, true),
        }
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

#[cfg(test)]
mod tests;
