// TODO: choose a better name.
pub trait ValuesBetweenExist: Ord {
    fn values_between_exist(lower: &Self, upper: &Self) -> bool;
}
macro_rules! impl_with_range {
    ($($ty:ty)*) => {
        $(
            impl ValuesBetweenExist for $ty {
                fn values_between_exist(lower: &Self, upper: &Self) -> bool {
                    (*lower..=<$ty>::MAX).nth(1).map_or(false, |x| x < *upper)
                }
            }
        )*
    };
}

impl<T: ValuesBetweenExist> ValuesBetweenExist for &T {
    fn values_between_exist(lower: &Self, upper: &Self) -> bool {
        T::values_between_exist(lower, upper)
    }
}

impl_with_range!(
    u8 u16 u32 u64 u128 usize
    i8 i16 i32 i64 i128 isize
    char
);

// TODO: impl for String, str, Instant, SystemTime, &mut T, [T], [T; N], Rc<T>, NonZero<T>, ...

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ints() {
        macro_rules! assert_impls {
            ($($ty:ty)*) => {
                $(
                    assert!(!<$ty>::values_between_exist(&<$ty>::MIN, &<$ty>::MIN));
                    assert!(!<$ty>::values_between_exist(&<$ty>::MIN, &(<$ty>::MIN + 1)));
                    assert!(<$ty>::values_between_exist(&<$ty>::MIN, &(<$ty>::MIN + 2)));

                    assert!(!<$ty>::values_between_exist(&10, &9));
                    assert!(!<$ty>::values_between_exist(&10, &10));
                    assert!(!<$ty>::values_between_exist(&10, &11));
                    assert!(<$ty>::values_between_exist(&10, &12));

                    assert!(!<$ty>::values_between_exist(&<$ty>::MAX, &<$ty>::MAX));
                    assert!(!<$ty>::values_between_exist(&(<$ty>::MAX - 1), &<$ty>::MAX));
                    assert!(<$ty>::values_between_exist(&(<$ty>::MAX - 2), &<$ty>::MAX));
                )*
            };
        }

        assert_impls!(
            u8 u16 u32 u64 u128 usize
            i8 i16 i32 i64 i128 isize
        );
    }

    #[test]
    fn char() {
        const BEFORE_FIRST_SURROGATE: char = '\u{D7FF}';
        const AFTER_LAST_SURROGATE: char = '\u{E000}';

        fn nth_next_char(from: char, nth: usize) -> char {
            (from..=char::MAX).nth(nth).unwrap()
        }
        fn nth_prev_char(to: char, nth: usize) -> char {
            (char::MIN..=to).nth_back(nth).unwrap()
        }

        assert!(!char::values_between_exist(
            &char::MIN,
            &nth_next_char(char::MIN, 0)
        ));
        assert!(!char::values_between_exist(
            &char::MIN,
            &nth_next_char(char::MIN, 1)
        ));
        assert!(char::values_between_exist(
            &char::MIN,
            &nth_next_char(char::MIN, 2)
        ));

        assert!(!char::values_between_exist(&'\u{B}', &'\u{A}'));
        assert!(!char::values_between_exist(&'\u{B}', &'\u{B}'));
        assert!(!char::values_between_exist(&'\u{B}', &'\u{C}'));
        assert!(char::values_between_exist(&'\u{B}', &'\u{D}'));

        assert!(!char::values_between_exist(
            &nth_prev_char(char::MAX, 0),
            &char::MAX
        ));
        assert!(!char::values_between_exist(
            &nth_prev_char(char::MAX, 1),
            &char::MAX
        ));
        assert!(char::values_between_exist(
            &nth_prev_char(char::MAX, 2),
            &char::MAX
        ));

        assert!(!char::values_between_exist(
            &nth_prev_char(BEFORE_FIRST_SURROGATE, 1),
            &BEFORE_FIRST_SURROGATE
        ));
        assert!(char::values_between_exist(
            &nth_prev_char(BEFORE_FIRST_SURROGATE, 1),
            &AFTER_LAST_SURROGATE
        ));
        assert!(!char::values_between_exist(
            &BEFORE_FIRST_SURROGATE,
            &AFTER_LAST_SURROGATE
        ));
        assert!(char::values_between_exist(
            &BEFORE_FIRST_SURROGATE,
            &nth_next_char(AFTER_LAST_SURROGATE, 1)
        ));
        assert!(!char::values_between_exist(
            &AFTER_LAST_SURROGATE,
            &nth_next_char(AFTER_LAST_SURROGATE, 1)
        ));
    }
}
