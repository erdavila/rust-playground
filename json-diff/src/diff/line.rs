use serde_json::Value;

use crate::comparison::{Comparison, ScalarPair, ScalarsComparison, Side};
use crate::diff::{
    ComparisonEntry, ComparisonEntryValue, Container, ContainerType, ContainersComparison, Diff,
    IntoContainersComparison as _, Sides,
};
use crate::token::{PutToken, Token};

pub(crate) fn tokenize<T: PutToken>(output: T, comparison: Comparison) -> Result<(), T::Error> {
    let mut line_diff = LineDiff { output };
    line_diff.put_comparison_tokens(0, None, comparison, Sides::None)
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
                    this.put_value_line_tokens(indent, key.clone(), value, commas.contains(side))
                })
            }
            Comparison::Arrays(arrays_cmp) => {
                let containers_cmp = arrays_cmp.into_containers_comparison();
                self.put_containers_comparison_line_tokens(indent, key, containers_cmp, commas)
            }
            Comparison::Objects(objects_cmp) => {
                let containers_cmp = objects_cmp.into_containers_comparison();
                self.put_containers_comparison_line_tokens(indent, key, containers_cmp, commas)
            }
            Comparison::DifferentTypes(left, right) => {
                self.put_both_sides_tokens(left, right, |this, value, side| {
                    this.put_value_line_tokens(indent, key.clone(), value, commas.contains(side))
                })
            }
        }
    }

    fn put_value_line_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        value: Value,
        comma: bool,
    ) -> Result<(), T::Error> {
        self.put_prefix_tokens(indent, key)?;
        self.put_value_tokens(indent, value, comma)?;
        self.output.put_token(Token::NewLine)
    }

    fn put_container_line_tokens<I>(
        &mut self,
        indent: usize,
        key: Option<String>,
        container: Container<I>,
        comma: bool,
    ) -> Result<(), T::Error>
    where
        I: IntoIterator<Item = (Option<String>, Value)>,
    {
        self.put_prefix_tokens(indent, key)?;
        self.put_container_tokens(indent, container)?;
        if comma {
            self.output.put_token(Token::Comma)?;
        }
        self.output.put_token(Token::NewLine)
    }

    fn put_containers_comparison_line_tokens<I>(
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
                this.put_empty_container_line_tokens(type_, indent, key.clone(), comma)
            }),
            (true, false) => {
                self.put_sided_tokens(Side::Left, |this| {
                    this.put_empty_container_line_tokens(
                        type_,
                        indent,
                        key,
                        commas.contains(Side::Left),
                    )
                })?;

                self.put_sided_tokens(Side::Right, |this| {
                    let container = containers_cmp.into_one_side(Side::Right);
                    this.put_container_line_tokens(
                        indent,
                        None,
                        container,
                        commas.contains(Side::Right),
                    )
                })
            }
            (false, true) => {
                self.put_sided_tokens(Side::Left, |this| {
                    let container = containers_cmp.into_one_side(Side::Left);
                    this.put_container_line_tokens(
                        indent,
                        None,
                        container,
                        commas.contains(Side::Left),
                    )
                })?;

                self.put_sided_tokens(Side::Right, |this| {
                    this.put_empty_container_line_tokens(
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
                            let commas = Sides::from_bools(left_count > 0, right_count > 0);
                            self.put_comparison_tokens(indent + 1, key, comparison, commas)?;
                        }
                        ComparisonEntryValue::OneSideOnly(value, side) => {
                            let count = match side {
                                Side::Left => &mut left_count,
                                Side::Right => &mut right_count,
                            };
                            *count -= 1;

                            self.put_sided_tokens(side, |this| {
                                this.put_value_line_tokens(indent + 1, key, value, *count > 0)
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

    fn put_empty_container_line_tokens(
        &mut self,
        type_: ContainerType,
        indent: usize,
        key: Option<String>,
        comma: bool,
    ) -> Result<(), T::Error> {
        self.put_prefix_tokens(indent, key)?;
        self.put_empty_container_tokens(type_)?;

        if comma {
            self.output.put_token(Token::Comma)?;
        }

        self.output.put_token(Token::NewLine)
    }

    fn put_line_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        content: impl IntoIterator<Item = Token>,
        comma: bool,
    ) -> Result<(), T::Error> {
        self.put_prefix_tokens(indent, key)?;

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
impl<T: PutToken> Diff<T> for LineDiff<T> {
    fn output(&mut self) -> &mut T {
        &mut self.output
    }
}

#[cfg(test)]
mod tests;
