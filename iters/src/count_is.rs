use std::cell::RefCell;

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
        let mut it = self.0.borrow_mut();
        let mut count = 0;

        loop {
            if it.next().is_some() {
                count += 1;
                if count > *other {
                    return false;
                }
            } else {
                return count == *other;
            }
        }
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
}
