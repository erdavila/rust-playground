type CmpOrdering = std::cmp::Ordering;

pub trait Compare<T>: Iterator<Item = T> + Sized {
    fn compare(mut self, other: impl IntoIterator<Item = T>) -> Comparison
    where
        T: Ord,
    {
        fn unequal_comparison(
            elements_were_compared: bool,
            mut compared_elements: usize,
            ordering: fn(Reason) -> Ordering,
        ) -> Comparison {
            let reason = if elements_were_compared {
                compared_elements += 1;
                Reason::UnequalElements
            } else {
                Reason::UnequalLengths
            };

            let ordering = ordering(reason);

            Comparison {
                compared_elements,
                ordering,
            }
        }

        let mut other = other.into_iter();

        let mut compared_elements = 0;
        loop {
            let self_next = self.next();
            let other_next = other.next();

            match self_next.cmp(&other_next) {
                CmpOrdering::Less => {
                    debug_assert!(other_next.is_some());
                    return unequal_comparison(
                        self_next.is_some(),
                        compared_elements,
                        Ordering::Less,
                    );
                }
                CmpOrdering::Equal => {
                    if self_next.is_none() {
                        return Comparison {
                            compared_elements,
                            ordering: Ordering::Equal,
                        };
                    }

                    compared_elements += 1;
                }
                CmpOrdering::Greater => {
                    debug_assert!(self_next.is_some());
                    return unequal_comparison(
                        other_next.is_some(),
                        compared_elements,
                        Ordering::Greater,
                    );
                }
            }
        }
    }
}
impl<T, I> Compare<T> for I where I: Iterator<Item = T> {}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Comparison {
    pub compared_elements: usize,
    pub ordering: Ordering,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Ordering {
    Less(Reason),
    Equal,
    Greater(Reason),
}
impl Ordering {
    #[must_use]
    pub fn to_cmp_ordering(self) -> CmpOrdering {
        match self {
            Ordering::Less(_) => CmpOrdering::Less,
            Ordering::Equal => CmpOrdering::Equal,
            Ordering::Greater(_) => CmpOrdering::Greater,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Reason {
    UnequalLengths,
    UnequalElements,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(&[], &[], Comparison { compared_elements: 0, ordering: Ordering::Equal })]
    #[case(&[1, 1, 1], &[2, 2, 2], Comparison { compared_elements: 1, ordering: Ordering::Less(Reason::UnequalElements) })]
    #[case(&[1, 2, 2], &[1, 2, 3], Comparison { compared_elements: 3, ordering: Ordering::Less(Reason::UnequalElements) })]
    #[case(&[2, 2, 2], &[1, 1, 1], Comparison { compared_elements: 1, ordering: Ordering::Greater(Reason::UnequalElements) })]
    #[case(&[1, 2, 3], &[1, 2, 2], Comparison { compared_elements: 3, ordering: Ordering::Greater(Reason::UnequalElements) })]
    #[case(&[1, 1], &[1, 1, 1], Comparison { compared_elements: 2, ordering: Ordering::Less(Reason::UnequalLengths) })]
    #[case(&[1, 1, 1], &[1, 1], Comparison { compared_elements: 2, ordering: Ordering::Greater(Reason::UnequalLengths) })]
    fn compare(#[case] a: &[i32], #[case] b: &[i32], #[case] expected: Comparison) {
        let output = a.into_iter().compare(b);

        assert_eq!(output, expected);
    }
}
