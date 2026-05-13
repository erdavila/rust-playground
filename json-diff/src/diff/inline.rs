use serde_json::Value;

use crate::comparison::{Comparison, ScalarPair, ScalarsComparison, Side};
use crate::diff::{
    ComparisonEntry, ComparisonEntryValue, ContainersComparison, Diff,
    IntoContainersComparison as _, Sides,
};
use crate::token::{PutToken, Token};

pub(crate) fn tokenize<T: PutToken>(output: T, comparison: Comparison) -> Result<(), T::Error> {
    let mut inline_diff = InlineDiff { output };
    inline_diff.put_comparison_tokens(0, None, comparison, Sides::None)
}

struct InlineDiff<T> {
    output: T,
}
impl<T: PutToken> InlineDiff<T> {
    fn put_comparison_tokens(
        &mut self,
        indent: usize,
        key: Option<String>,
        comparison: Comparison,
        commas: Sides,
    ) -> Result<(), T::Error> {
        self.put_prefix_tokens(indent, key)?;

        match comparison {
            Comparison::Scalars(ScalarsComparison::Same(scalar)) => {
                self.output.put_token(scalar.into())?;
                self.put_possibly_sided_comma(commas)?;
            }
            Comparison::Scalars(ScalarsComparison::Different(scalar_pair)) => {
                let (left, right) = match scalar_pair {
                    ScalarPair::Bools(left, right) => (left.into(), right.into()),
                    ScalarPair::Numbers(left, right) => (left.into(), right.into()),
                    ScalarPair::Strings(left, right) => (left.into(), right.into()),
                };

                self.put_both_sides_values_tokens(indent, left, right, commas)?;
            }
            Comparison::Arrays(arrays_cmp) => {
                let containers_cmp = arrays_cmp.into_containers_comparison();
                self.put_containers_comparison_tokens(indent, containers_cmp, commas)?;
            }
            Comparison::Objects(objects_cmp) => {
                let containers_cmp = objects_cmp.into_containers_comparison();
                self.put_containers_comparison_tokens(indent, containers_cmp, commas)?;
            }
            Comparison::DifferentTypes(left, right) => {
                self.put_both_sides_values_tokens(indent, left, right, commas)?;
            }
        }

        self.output.put_token(Token::NewLine)
    }

    fn put_containers_comparison_tokens<I>(
        &mut self,
        indent: usize,
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
            (true, true) => {
                self.put_empty_container_tokens(type_)?;
                self.put_possibly_sided_comma(commas)
            }
            (true, false) => {
                self.put_sided_tokens(Side::Left, |this| this.put_empty_container_tokens(type_))?;

                self.put_sided_tokens(Side::Right, |this| {
                    let container = containers_cmp.into_one_side(Side::Right);
                    this.put_container_tokens(indent, container)
                })
            }
            (false, true) => {
                self.put_sided_tokens(Side::Left, |this| {
                    let container = containers_cmp.into_one_side(Side::Left);
                    this.put_container_tokens(indent, container)
                })?;

                self.put_sided_tokens(Side::Right, |this| this.put_empty_container_tokens(type_))
            }
            (false, false) => {
                self.output.put_token(containers_cmp.type_.begin_token())?;
                self.output.put_token(Token::NewLine)?;

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

                            self.put_indent_tokens(indent + 1)?;
                            self.put_sided_tokens(side, |this| {
                                if let Some(key) = key {
                                    this.output.put_token(Token::Key(key))?;
                                }
                                this.put_value_tokens(indent + 1, value, *count > 0)
                            })?;
                            self.output.put_token(Token::NewLine)?;
                        }
                    }
                }

                self.put_indent_tokens(indent)?;
                self.output.put_token(containers_cmp.type_.end_token())?;
                self.put_possibly_sided_comma(commas)
            }
        }
    }

    fn put_both_sides_values_tokens(
        &mut self,
        indent: usize,
        left: Value,
        right: Value,
        commas: Sides,
    ) -> Result<(), T::Error> {
        for (value, side) in [(left, Side::Left), (right, Side::Right)] {
            self.put_sided_tokens(side, |this| {
                this.put_value_tokens(indent, value, commas == Sides::only(side))
            })?;
        }

        if commas == Sides::Both {
            self.output.put_token(Token::Comma)?;
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

    fn put_possibly_sided_comma(&mut self, commas: Sides) -> Result<(), T::Error> {
        match commas {
            Sides::None => Ok(()),
            Sides::LeftOnly => {
                self.put_sided_tokens(Side::Left, |this| this.output.put_token(Token::Comma))
            }
            Sides::RightOnly => {
                self.put_sided_tokens(Side::Right, |this| this.output.put_token(Token::Comma))
            }
            Sides::Both => self.output.put_token(Token::Comma),
        }
    }
}
impl<T: PutToken> Diff<T> for InlineDiff<T> {
    fn output(&mut self) -> &mut T {
        &mut self.output
    }
}

#[cfg(test)]
mod tests;
