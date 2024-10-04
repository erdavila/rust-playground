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
    pub fn eq(self, tested_count: usize) -> impl Evaluator {
        Eq::new(tested_count)
    }

    pub fn ne(self, tested_count: usize) -> impl Evaluator {
        Not::new(Eq::new(tested_count))
    }

    pub fn lt(self, tested_count: usize) -> impl Evaluator {
        Lt::new(tested_count)
    }

    pub fn gt(self, tested_count: usize) -> impl Evaluator {
        Gt::new(tested_count)
    }

    pub fn le(self, tested_count: usize) -> impl Evaluator {
        Not::new(Gt::new(tested_count))
    }

    pub fn ge(self, tested_count: usize) -> impl Evaluator {
        Not::new(Lt::new(tested_count))
    }
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
}
