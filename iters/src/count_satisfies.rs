pub mod evaluation;

use evaluation::{Eq, Evaluator, Gt, Lt, Not};

pub trait CountSatisfies: Iterator + Sized {
    fn count_satisfies<E>(mut self, f: impl FnOnce(N) -> E) -> bool
    where
        E: Evaluator,
    {
        f(N).evaluate(&mut self)
    }
}
impl<I> CountSatisfies for I where I: Iterator {}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct N;
impl N {
    #[must_use]
    pub fn eq(self, tested_count: usize) -> impl Evaluator {
        Eq::new(tested_count)
    }

    #[must_use]
    pub fn ne(self, tested_count: usize) -> impl Evaluator {
        Not::new(Eq::new(tested_count))
    }

    #[must_use]
    pub fn lt(self, tested_count: usize) -> impl Evaluator {
        Lt::new(tested_count)
    }

    #[must_use]
    pub fn gt(self, tested_count: usize) -> impl Evaluator {
        Gt::new(tested_count)
    }

    #[must_use]
    pub fn le(self, tested_count: usize) -> impl Evaluator {
        Not::new(Gt::new(tested_count))
    }

    #[must_use]
    pub fn ge(self, tested_count: usize) -> impl Evaluator {
        Not::new(Lt::new(tested_count))
    }
}

#[macro_export]
#[doc(hidden)]
#[expect(clippy::module_name_repetitions)]
macro_rules! __count_satisfies_condition_comparison {
    ($n:ident, ==, $compared:tt) => {
        $n.eq($compared)
    };
    ($n:ident, !=, $compared:tt) => {
        $n.ne($compared)
    };
    ($n:ident, <, $compared:tt) => {
        $n.lt($compared)
    };
    ($n:ident, >, $compared:tt) => {
        $n.gt($compared)
    };
    ($n:ident, <=, $compared:tt) => {
        $n.le($compared)
    };
    ($n:ident, >=, $compared:tt) => {
        $n.ge($compared)
    };
}

#[macro_export]
#[doc(hidden)]
#[expect(clippy::module_name_repetitions)]
macro_rules! __count_satisfies_condition_parse_or_rest {
    ($expr1:expr , $expr2:expr , && $n:ident $cmp:tt $compared:tt $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr1,
            ( $expr2 ).and( $crate::__count_satisfies_condition_comparison!($n, $cmp, $compared) ),
            $($rest)*
        )
    };
    ($expr1:expr , $expr2:expr , && ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr1,
            ( $expr2 ).and( $crate::__count_satisfies_condition_parse!($($token)+) ),
            $($rest)*
        )
    };
    ($expr1:expr , $expr2:expr , && ! ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr1,
            ( $expr2 ).and( ( $crate::__count_satisfies_condition_parse!($($token)+) ).not() ),
            $($rest)*
        )
    };
    ($expr1:expr , $expr2:expr , $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            ( $expr1 ).or( $expr2 ),
            $($rest)*
        )
    };
}

#[macro_export]
#[doc(hidden)]
#[expect(clippy::module_name_repetitions)]
macro_rules! __count_satisfies_condition_parse_rest {
    ($expr:expr , ) => {
        $expr
    };
    ($expr:expr , && $n:ident $cmp:tt $compared:tt $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            ( $expr ).and( $crate::__count_satisfies_condition_comparison!($n, $cmp, $compared) ),
            $($rest)*
        )
    };
    ($expr:expr , && ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            ( $expr ).and( $crate::__count_satisfies_condition_parse!($($token)+) ),
            $($rest)*
        )
    };
    ($expr:expr , && ! ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            ( $expr ).and(( $crate::__count_satisfies_condition_parse!($($token)+) ).not()),
            $($rest)*
        )
    };
    ($expr:expr , || $n:ident $cmp:tt $compared:tt $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr,
            $crate::__count_satisfies_condition_comparison!($n, $cmp, $compared),
            $($rest)*
        )
    };
    ($expr:expr , || ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr,
            ( $crate::__count_satisfies_condition_parse!($($token)+) ),
            $($rest)*
        )
    };
    ($expr:expr , || ! ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_or_rest!(
            $expr,
            ( $crate::__count_satisfies_condition_parse!($($token)+) ).not(),
            $($rest)*
        )
    };
}

#[macro_export]
#[doc(hidden)]
#[expect(clippy::module_name_repetitions)]
macro_rules! __count_satisfies_condition_parse {
    ($n:ident $cmp:tt $compared:tt $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            $crate::__count_satisfies_condition_comparison!($n, $cmp, $compared),
            $($rest)*
        )
    };
    (( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            $crate::__count_satisfies_condition_parse!($($token)+),
            $($rest)*
        )
    };
    (! ( $($token:tt)+ ) $($rest:tt)*) => {
        $crate::__count_satisfies_condition_parse_rest!(
            ( $crate::__count_satisfies_condition_parse!($($token)+) ).not(),
            $($rest)*
        )
    };
}

#[macro_export]
#[expect(clippy::module_name_repetitions)]
macro_rules! count_satisfies_condition {
    (|$n:ident| { $($token:tt)+ }) => {
        |$n| $crate::__count_satisfies_condition_parse!($($token)+)
    };
    (|$n:ident| $($token:tt)+) => {
        |$n| $crate::__count_satisfies_condition_parse!($($token)+)
    };
}

#[macro_export]
macro_rules! count_satisfies {
    ($iterator:expr, |$n:ident| $($token:tt)+) => {
        $crate::count_satisfies::CountSatisfies::count_satisfies($iterator, $crate::count_satisfies_condition!(|$n| $($token)+))
    };
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    macro_rules! assert_evaluation {
        (($real_count:literal elems).count_satisfies($expr:expr) should be $expected:literal, $consumed:literal) => {{
            let consumed = Rc::new(Cell::new(0_usize));
            let it = std::iter::repeat(()).take($real_count).inspect({
                let consumed = Rc::clone(&consumed);
                move |_| {
                    consumed.set(consumed.get() + 1);
                }
            });

            let output = it.count_satisfies($expr);

            assert_eq!(output, $expected);
            assert_eq!(consumed.get(), $consumed);
        }};
    }

    #[test]
    fn not() {
        assert_evaluation!((0 elems).count_satisfies(|n| n.lt(3).not()) should be false, 0);
        assert_evaluation!((1 elems).count_satisfies(|n| n.lt(3).not()) should be false, 1);
        assert_evaluation!((2 elems).count_satisfies(|n| n.lt(3).not()) should be false, 2);
        assert_evaluation!((3 elems).count_satisfies(|n| n.lt(3).not()) should be true, 3);
        assert_evaluation!((4 elems).count_satisfies(|n| n.lt(3).not()) should be true, 3);
    }

    #[test]
    fn or() {
        assert_evaluation!((0 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be true, 0);
        assert_evaluation!((1 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be true, 1);
        assert_evaluation!((2 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be false, 2);
        assert_evaluation!((3 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be true, 3);
        assert_evaluation!((4 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be false, 4);
        assert_evaluation!((5 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be true, 5);
        assert_evaluation!((6 elems).count_satisfies(|n| n.lt(2).or(n.eq(3)).or(n.ge(5))) should be true, 5);

        assert_evaluation!((0 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be true, 0);
        assert_evaluation!((1 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be true, 1);
        assert_evaluation!((2 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be false, 2);
        assert_evaluation!((3 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be true, 3);
        assert_evaluation!((4 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be false, 4);
        assert_evaluation!((5 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be true, 5);
        assert_evaluation!((6 elems).count_satisfies(|n| n.ge(5).or(n.eq(3)).or(n.lt(2))) should be true, 5);
    }

    #[test]
    fn and() {
        assert_evaluation!((0 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be false, 0);
        assert_evaluation!((1 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be false, 1);
        assert_evaluation!((2 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be true, 2);
        assert_evaluation!((3 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be false, 3);
        assert_evaluation!((4 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be true, 4);
        assert_evaluation!((5 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be false, 5);
        assert_evaluation!((6 elems).count_satisfies(|n| n.gt(1).and(n.ne(3)).and(n.le(4))) should be false, 5);

        assert_evaluation!((0 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be false, 0);
        assert_evaluation!((1 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be false, 1);
        assert_evaluation!((2 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be true, 2);
        assert_evaluation!((3 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be false, 3);
        assert_evaluation!((4 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be true, 4);
        assert_evaluation!((5 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be false, 5);
        assert_evaluation!((6 elems).count_satisfies(|n| n.le(4).and(n.ne(3)).and(n.gt(1))) should be false, 5);
    }

    #[test]
    fn condition_macro() {
        fn iter(len: usize) -> impl Iterator {
            std::iter::repeat(()).take(len)
        }

        for len in 0..5 {
            let expected = len < 2;
            assert_eq!(
                iter(len).count_satisfies(count_satisfies_condition!(|n| n < 2)),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| { n < 2 }),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| n < 2),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| (n < 2)),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| !(n < 2)),
                !expected,
                "len={len}"
            );

            let expected = len > 0 && len < 3 || len == 4;
            assert_eq!(
                count_satisfies!(iter(len), |n| n > 0 && n < 3 || n == 4),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| (n > 0) && (n < 3) || (n == 4)),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| !(n <= 0) && !(n >= 3) || !(n != 4)),
                expected,
                "len={len}"
            );

            let expected = len == 4 || len > 0 && len < 3;
            assert_eq!(
                count_satisfies!(iter(len), |n| n == 4 || n > 0 && n < 3),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| (n == 4) || (n > 0) && (n < 3)),
                expected,
                "len={len}"
            );
            assert_eq!(
                count_satisfies!(iter(len), |n| !(n != 4) || !(n <= 0) && !(n >= 3)),
                expected,
                "len={len}"
            );
        }
    }
}
