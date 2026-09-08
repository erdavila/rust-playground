use core::fmt;
use core::fmt::{Debug, Write as _};
use core::iter::once;
use std::cmp::PartialEq;

use interval_map::IntervalMap;
use interval_map::interval::Endpoint::{Closed, Infinity, Open};
use interval_map::interval::{Interval, IntervalLike as _};

#[test]
fn empty() {
    let map: IntervalMap<i32, char> = IntervalMap::new();

    assert_eq!(map.get(&10), None);
}

mod insert_single {
    use super::*;

    #[test]
    fn open_open() {
        let mut map = IntervalMap::new();

        // (7, 10)
        map.insert((Open(7), Open(10)), 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), None);
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), None);
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn open_closed() {
        let mut map = IntervalMap::new();

        // (7, 10]
        map.insert((Open(7), Closed(10)), 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), None);
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn open_infinity() {
        let mut map = IntervalMap::new();

        // (7, +∞)
        map.insert((Open(7), Infinity), 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), None);
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), Some(&'a'));
    }

    #[test]
    fn closed_open() {
        let mut map = IntervalMap::new();

        // [7, 10)
        map.insert(7..10, 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), None);
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn closed_closed() {
        let mut map = IntervalMap::new();

        // [7, 10]
        map.insert(7..=10, 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn closed_infinity() {
        let mut map = IntervalMap::new();

        // [7, +∞)
        map.insert(7.., 'a');

        assert_eq!(map.get(&6), None);
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), Some(&'a'));
    }

    #[test]
    fn infinity_open() {
        let mut map = IntervalMap::new();

        // (-∞, 10)
        map.insert(..10, 'a');

        assert_eq!(map.get(&6), Some(&'a'));
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), None);
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn infinity_closed() {
        let mut map = IntervalMap::new();

        // (-∞, 10]
        map.insert(..=10, 'a');

        assert_eq!(map.get(&6), Some(&'a'));
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), None);
    }

    #[test]
    fn infinity_infinity() {
        let mut map = IntervalMap::new();

        // (-∞, +∞)
        map.insert(.., 'a');

        assert_eq!(map.get(&6), Some(&'a'));
        assert_eq!(map.get(&7), Some(&'a'));
        assert_eq!(map.get(&8), Some(&'a'));
        assert_eq!(map.get(&9), Some(&'a'));
        assert_eq!(map.get(&10), Some(&'a'));
        assert_eq!(map.get(&11), Some(&'a'));
    }
}

mod insert_multiple {
    use super::*;

    const MIN_K: i32 = 10;
    const MAX_K: i32 = 20;

    // TODO: remove it
    // #[test]
    fn debug_case() {
        // (10, 11] -> 'a'; (10, 12] -> 'a'; (10, 11] -> 'a'; 12
        test_case([
            (Interval::new(Open(10), Closed(11)), 'a'),
            (Interval::new(Open(10), Closed(12)), 'a'),
            (Interval::new(Open(10), Closed(11)), 'a'),
        ]);
    }

    #[test]
    fn one_value() {
        test_cases('a', 'a', 'a');
    }

    #[test]
    fn two_values() {
        test_cases('a', 'a', 'b');
    }

    #[test]
    fn three_values() {
        test_cases('a', 'b', 'c');
    }

    fn test_cases<V>(value_1: V, value_2: V, value_3: V)
    where
        V: Copy + PartialEq + Debug,
    {
        // TODO: remove duplication with other tests.
        let endpoints = (MIN_K..=MAX_K)
            .flat_map(|n| [Open(n), Closed(n)])
            .chain(once(Infinity));

        // TODO: remove duplication with other tests.
        let intervals = endpoints
            .clone()
            .flat_map(|left| endpoints.clone().map(move |right| Interval { left, right }))
            .filter(|interval| !interval.is_empty());

        //  TODO: remove it
        let mut count = 0;
        // const LIMIT: u32 = 841;
        const LIMIT: u32 = 1;

        for interval_1 in intervals.clone() {
            for interval_2 in intervals.clone() {
                for interval_3 in intervals.clone() {
                    if count >= LIMIT {
                        // panic!();
                        return;
                    }
                    count += 1;

                    test_case([
                        (interval_1, value_1),
                        (interval_2, value_2),
                        (interval_3, value_3),
                    ]);
                }
            }
        }

        // TODO: remove it
        panic!("FINISHED");
    }

    fn test_case<V, const N: usize>(entries: [(Interval<i32>, V); N])
    where
        V: Copy + PartialEq + Debug,
    {
        let mut map = IntervalMap::new();

        for (i, (interval, value)) in entries.into_iter().enumerate() {
            // TODO: remove it
            // println!(">>>> Inserting [{i}] {:?}", entry_debug(&interval, &value));

            map.insert(interval, value);

            // TODO: remove it
            // println!("  >>>> {map:?}");
        }

        let dbg = fmt::from_fn(|f| {
            for (interval, value) in entries {
                write!(f, "{:?}; ", entry_debug(&interval, &value))?;
            }
            Ok(())
        });

        // TODO: remove it
        // println!("{dbg}");

        'n: for n in MIN_K - 1..=MAX_K + 1 {
            let value = map.get(&n);

            for (interval, expected_value) in entries.into_iter().rev() {
                if interval.contains(&n) {
                    assert_eq!(value, Some(&expected_value), "{dbg}{n}");
                    continue 'n;
                }
            }

            assert_eq!(value, None, "{dbg}");
        }

        // TODO: check the intervals in the map.
    }
}

fn entry_debug<K: Debug, V: Debug>(interval: &Interval<K>, value: &V) -> impl Debug {
    fmt::from_fn(move |f| write!(f, "{interval:?} -> {value:?}"))
}
