use std::{cell::RefCell, ops::DerefMut};

use crate::count_satisfies::evaluation::{Eq, Evaluator, Gt, Lt, Not};

pub trait CountIs: Iterator + Sized {
    fn count_is(self) -> CountIsEvaluator<Self> {
        CountIsEvaluator(RefCell::new(self))
    }
}
impl<I> CountIs for I where I: Iterator {}

#[derive(Clone, Debug)]
pub struct CountIsEvaluator<I>(RefCell<I>)
where
    I: Iterator;
impl<I> PartialEq<usize> for CountIsEvaluator<I>
where
    I: Iterator,
{
    fn eq(&self, other: &usize) -> bool {
        let e = Eq::new(*other);
        e.evaluate(&mut *self.0.borrow_mut())
    }
}
impl<I> PartialOrd<usize> for CountIsEvaluator<I>
where
    I: Iterator,
{
    fn partial_cmp(&self, other: &usize) -> Option<std::cmp::Ordering> {
        let mut it = self.0.borrow_mut();
        let mut consumed;

        if *other == 0 {
            consumed = 0;

            if it.next().is_some() {
                consumed += 1;
            }
        } else {
            consumed = *other - 1;
            let mut it = it.deref_mut().skip(consumed);

            if it.next().is_some() {
                consumed += 1;

                if it.next().is_some() {
                    consumed += 1;
                }
            }
        };

        consumed.partial_cmp(other)
    }

    fn lt(&self, other: &usize) -> bool {
        let e = Lt::new(*other);
        e.evaluate(&mut *self.0.borrow_mut())
    }

    fn gt(&self, other: &usize) -> bool {
        let e = Gt::new(*other);
        e.evaluate(&mut *self.0.borrow_mut())
    }

    fn le(&self, other: &usize) -> bool {
        let e = Not::new(Gt::new(*other));
        e.evaluate(&mut *self.0.borrow_mut())
    }

    fn ge(&self, other: &usize) -> bool {
        let e = Not::new(Lt::new(*other));
        e.evaluate(&mut *self.0.borrow_mut())
    }
}

#[cfg(test)]
mod tests {
    use std::{cell::Cell, rc::Rc};

    use super::*;

    macro_rules! assert_op {
        (($real_count:literal elems).count_is() $op:tt $tested_count:literal, $consumed:literal) => {
            {
                let consumed = Rc::new(Cell::new(0_usize));
                let it = std::iter::repeat(()).take($real_count).inspect({
                    let consumed = Rc::clone(&consumed);
                    move |_| {
                        consumed.set(consumed.get() + 1);
                    }
                });
                let expected = $real_count $op $tested_count;

                let output = it.count_is() $op $tested_count;

                assert_eq!(output, expected);
                assert_eq!(consumed.get(), $consumed);
            }
        };
        (($real_count:literal elems).count_is().partial_cmp(&$tested_count:literal), $consumed:literal) => {
            {
                let consumed = Rc::new(Cell::new(0_usize));
                let it = std::iter::repeat(()).take($real_count).inspect({
                    let consumed = Rc::clone(&consumed);
                    move |_| {
                        consumed.set(consumed.get() + 1);
                    }
                });
                let expected = $real_count.partial_cmp(&$tested_count);

                let output = it.count_is().partial_cmp(&$tested_count);

                assert_eq!(output, expected);
                assert_eq!(consumed.get(), $consumed);
            }
        };
    }

    #[test]
    fn eq() {
        assert_op!((0 elems).count_is() == 0, 0);
        assert_op!((0 elems).count_is() == 1, 0);

        assert_op!((1 elems).count_is() == 0, 1);
        assert_op!((1 elems).count_is() == 1, 1);
        assert_op!((1 elems).count_is() == 2, 1);

        assert_op!((2 elems).count_is() == 0, 1);
        assert_op!((2 elems).count_is() == 1, 2);
        assert_op!((2 elems).count_is() == 2, 2);
        assert_op!((2 elems).count_is() == 3, 2);

        assert_op!((3 elems).count_is() == 0, 1);
        assert_op!((3 elems).count_is() == 1, 2);
        assert_op!((3 elems).count_is() == 2, 3);
        assert_op!((3 elems).count_is() == 3, 3);
        assert_op!((3 elems).count_is() == 4, 3);
    }

    #[test]
    fn ne() {
        assert_op!((0 elems).count_is() != 0, 0);
        assert_op!((0 elems).count_is() != 1, 0);

        assert_op!((1 elems).count_is() != 0, 1);
        assert_op!((1 elems).count_is() != 1, 1);
        assert_op!((1 elems).count_is() != 2, 1);

        assert_op!((2 elems).count_is() != 0, 1);
        assert_op!((2 elems).count_is() != 1, 2);
        assert_op!((2 elems).count_is() != 2, 2);
        assert_op!((2 elems).count_is() != 3, 2);

        assert_op!((3 elems).count_is() != 0, 1);
        assert_op!((3 elems).count_is() != 1, 2);
        assert_op!((3 elems).count_is() != 2, 3);
        assert_op!((3 elems).count_is() != 3, 3);
        assert_op!((3 elems).count_is() != 4, 3);
    }

    #[test]
    fn lt() {
        assert_op!((0 elems).count_is() < 0, 0);
        assert_op!((0 elems).count_is() < 1, 0);

        assert_op!((1 elems).count_is() < 0, 0);
        assert_op!((1 elems).count_is() < 1, 1);
        assert_op!((1 elems).count_is() < 2, 1);

        assert_op!((2 elems).count_is() < 0, 0);
        assert_op!((2 elems).count_is() < 1, 1);
        assert_op!((2 elems).count_is() < 2, 2);
        assert_op!((2 elems).count_is() < 3, 2);

        assert_op!((3 elems).count_is() < 0, 0);
        assert_op!((3 elems).count_is() < 1, 1);
        assert_op!((3 elems).count_is() < 2, 2);
        assert_op!((3 elems).count_is() < 3, 3);
        assert_op!((3 elems).count_is() < 4, 3);
    }

    #[test]
    fn gt() {
        assert_op!((0 elems).count_is() > 0, 0);
        assert_op!((0 elems).count_is() > 1, 0);

        assert_op!((1 elems).count_is() > 0, 1);
        assert_op!((1 elems).count_is() > 1, 1);
        assert_op!((1 elems).count_is() > 2, 1);

        assert_op!((2 elems).count_is() > 0, 1);
        assert_op!((2 elems).count_is() > 1, 2);
        assert_op!((2 elems).count_is() > 2, 2);
        assert_op!((2 elems).count_is() > 3, 2);

        assert_op!((3 elems).count_is() > 0, 1);
        assert_op!((3 elems).count_is() > 1, 2);
        assert_op!((3 elems).count_is() > 2, 3);
        assert_op!((3 elems).count_is() > 3, 3);
        assert_op!((3 elems).count_is() > 4, 3);
    }

    #[test]
    fn le() {
        assert_op!((0 elems).count_is() <= 0, 0);
        assert_op!((0 elems).count_is() <= 1, 0);

        assert_op!((1 elems).count_is() <= 0, 1);
        assert_op!((1 elems).count_is() <= 1, 1);
        assert_op!((1 elems).count_is() <= 2, 1);

        assert_op!((2 elems).count_is() <= 0, 1);
        assert_op!((2 elems).count_is() <= 1, 2);
        assert_op!((2 elems).count_is() <= 2, 2);
        assert_op!((2 elems).count_is() <= 3, 2);

        assert_op!((3 elems).count_is() <= 0, 1);
        assert_op!((3 elems).count_is() <= 1, 2);
        assert_op!((3 elems).count_is() <= 2, 3);
        assert_op!((3 elems).count_is() <= 3, 3);
        assert_op!((3 elems).count_is() <= 4, 3);
    }

    #[test]
    fn ge() {
        assert_op!((0 elems).count_is() >= 0, 0);
        assert_op!((0 elems).count_is() >= 1, 0);

        assert_op!((1 elems).count_is() >= 0, 0);
        assert_op!((1 elems).count_is() >= 1, 1);
        assert_op!((1 elems).count_is() >= 2, 1);

        assert_op!((2 elems).count_is() >= 0, 0);
        assert_op!((2 elems).count_is() >= 1, 1);
        assert_op!((2 elems).count_is() >= 2, 2);
        assert_op!((2 elems).count_is() >= 3, 2);

        assert_op!((3 elems).count_is() >= 0, 0);
        assert_op!((3 elems).count_is() >= 1, 1);
        assert_op!((3 elems).count_is() >= 2, 2);
        assert_op!((3 elems).count_is() >= 3, 3);
        assert_op!((3 elems).count_is() >= 4, 3);
    }

    #[test]
    fn partial_cmp() {
        assert_op!((0 elems).count_is().partial_cmp(&0), 0);
        assert_op!((0 elems).count_is().partial_cmp(&1), 0);

        assert_op!((1 elems).count_is().partial_cmp(&0), 1);
        assert_op!((1 elems).count_is().partial_cmp(&1), 1);
        assert_op!((1 elems).count_is().partial_cmp(&2), 1);

        assert_op!((2 elems).count_is().partial_cmp(&0), 1);
        assert_op!((2 elems).count_is().partial_cmp(&1), 2);
        assert_op!((2 elems).count_is().partial_cmp(&2), 2);
        assert_op!((2 elems).count_is().partial_cmp(&3), 2);

        assert_op!((3 elems).count_is().partial_cmp(&0), 1);
        assert_op!((3 elems).count_is().partial_cmp(&1), 2);
        assert_op!((3 elems).count_is().partial_cmp(&2), 3);
        assert_op!((3 elems).count_is().partial_cmp(&3), 3);
        assert_op!((3 elems).count_is().partial_cmp(&4), 3);
    }
}
