use core::cmp::PartialEq;
use core::fmt;
use core::fmt::Debug;
use core::iter::once;

use interval_map::IntervalMap;
use interval_map::interval::Interval;
use interval_map::interval::LeftEndpoint::{
    Closed as LClosed, Infinity as LInfinity, Open as LOpen,
};
use interval_map::interval::RightEndpoint::{
    Closed as RClosed, Infinity as RInfinity, Open as ROpen,
};

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
        map.insert((LOpen(7), ROpen(10)), 'a');

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
        map.insert((LOpen(7), RClosed(10)), 'a');

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
        map.insert((LOpen(7), RInfinity), 'a');

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
    use interval_map::RawEntry;
    use interval_map::interval::IntervalLike;

    use super::*;

    // K_DELTA = 2 is the min number to exercise all cases.
    const K_DELTA: i32 = 2;

    const MIN_K: i32 = 0;
    const MAX_K: i32 = MIN_K + K_DELTA - 1;

    // #[test]
    // fn debug_case() {
    //     // (10, 11]: 'a'; (10, 11]: 'a'; [11, 11]: 'b';
    //     let entries = [
    //         (Interval::new(LOpen(10), RClosed(11)), 'a'),
    //         (Interval::new(LOpen(10), RClosed(11)), 'a'),
    //         (Interval::new(LClosed(11), RClosed(11)), 'b'),
    //     ];

    //     let check_overwritten = true;

    //     test_case(entries, check_overwritten);
    // }

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
        V: PartialEq + Copy + Debug,
    {
        const CASES_LIMIT: Option<u32> = None;

        // TODO: remove duplication with other tests.
        let lefts = (MIN_K..=MAX_K)
            .flat_map(|n| [LOpen(n), LClosed(n)])
            .chain(once(LInfinity));
        let rights = (MIN_K..=MAX_K)
            .flat_map(|n| [ROpen(n), RClosed(n)])
            .chain(once(RInfinity));

        // TODO: remove duplication with other tests.
        let intervals = lefts
            .clone()
            .flat_map(|left| rights.clone().map(move |right| Interval { left, right }))
            .filter(|interval| interval.left <= interval.right);

        let mut cases_count = 0;

        for interval_1 in intervals.clone() {
            for interval_2 in intervals.clone() {
                for interval_3 in intervals.clone() {
                    assert!(
                        CASES_LIMIT.is_none_or(|limit| cases_count < limit),
                        "ABORTING after {cases_count} COMBINATIONS"
                    );
                    cases_count += 1;

                    let entries = [
                        (interval_1, value_1),
                        (interval_2, value_2),
                        (interval_3, value_3),
                    ];

                    test_case(entries, false);
                    test_case(entries, true);
                }
            }
        }

        /*
           K_DELTA -> combinations
           0 -> 1
           1 -> 216
           2 -> 3375
           3 -> 21952
           4 -> 91125
           5 -> 287496
        */
        eprintln!("ALL {cases_count} COMBINATIONS TESTED (K_DELTA = {K_DELTA})");
    }

    fn test_case<V, const N: usize>(entries: [(Interval<i32>, V); N], check_overwritten: bool)
    where
        V: Copy + PartialEq + Debug,
    {
        let dbg = fmt::from_fn(|f| {
            for (interval, value) in entries {
                write!(f, "{:?}; ", RawEntry::new(interval, value))?;
            }
            Ok(())
        });

        let mut map = IntervalMap::new();

        for (interval, value) in entries {
            let before: Vec<_> = map.dbg_entries().map(RawEntry::from).collect();

            let overwritten = map.insert(interval, value);

            let inserted = RawEntry { interval, value };

            /*
               Iterating the iterator returned by `insert` may clone additional data.
               We optionally iterate it, so that we can exercise both cases.
            */
            let overwritten =
                check_overwritten.then(|| overwritten.map(RawEntry::from).collect::<Vec<_>>());

            let after: Vec<RawEntry<_, _>> = map.dbg_entries().map(RawEntry::from).collect();

            if let Some(overwritten) = overwritten {
                let mut previous: Option<RawEntry<i32, V>> = None;
                for over in overwritten {
                    // The interval must be proper.
                    assert!(over.left() <= over.right());

                    let over_is_relaxed_subinterval_of = |interval: Interval<i32>| {
                        /*
                            over:     {OL, OR}
                            interval: {IL, IR}
                        */

                        // IL <= OL or {OL, IL@ is empty.
                        let left = interval.left <= over.left()
                            || Interval::new(
                                *over.left(),
                                interval.left.opposite().unwrap().copied(),
                            )
                            .is_empty();

                        // OR <= IR or @IR, OR} is empty.
                        let right = over.right() <= interval.right
                            || Interval::new(
                                interval.right.opposite().unwrap().copied(),
                                *over.right(),
                            )
                            .is_empty();

                        left && right
                    };

                    // Each overwritten interval must be a sub-interval of the inserted interval.
                    assert!(over_is_relaxed_subinterval_of(inserted.interval));

                    // Each overwritten entry must have a sub-interval of one of the previous
                    // entries.
                    assert!(before.iter().any(|bef| {
                        over.interval.is_subinterval_of(&bef.interval) && over.value == bef.value
                    }));

                    // Each overwritten interval must be a sub-interval of one of the resulting
                    // intervals.
                    assert!(
                        after
                            .iter()
                            .any(|bef| { over_is_relaxed_subinterval_of(bef.interval) })
                    );

                    if let Some(prev) = previous {
                        // Entries must be in order, and not intersect.
                        assert!(over.left() > prev.right());
                    }

                    previous = Some(over);
                }
            }

            // The inserted entry must be in the resulting entries.
            assert!(
                inserted.interval.is_empty()
                    || after.iter().any(|entry| {
                        inserted.interval.is_subinterval_of(&entry.interval)
                            && inserted.value == entry.value
                    })
            );

            let mut previous: Option<RawEntry<i32, V>> = None;
            for entry in &after {
                if let Some(prev) = previous {
                    // Entries must be in order, and not intersect.
                    assert!(entry.left() > prev.right());

                    // If consecutive entries have the same value, there must be a non-empty gap
                    // between them.
                    if prev.value == entry.value {
                        let between = Interval {
                            left: prev.right().opposite().unwrap(),
                            right: entry.left().opposite().unwrap(),
                        };
                        assert!(!between.is_empty());
                    }
                }

                previous = Some(*entry);
            }
        }

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
    }

    #[test]
    fn single_insert_multiple_overwrites() {
        /*
         * BEFORE
         *     in map:
         *                 [10,     14)                     -> 'a'
         *                             [16, 18)             -> 'b'
         *                                     [20,     24) -> 'c'
         *     inserting:      [12,                 22)     -> 'x'
         *
         * EXPECTED
         *     in map:
         *                 [10, 12)                         -> 'a'
         *                     [12,                 22)     -> 'x'
         *                                         [22, 24) -> 'c'
         *     output:
         *                     [12, 14)                     -> 'a'
         *                             [16, 18)             -> 'b'
         *                                     [20, 22)     -> 'c'
         */

        let mut map = IntervalMap::new();
        map.insert(10..14, 'a');
        map.insert(16..18, 'b');
        map.insert(20..24, 'c');

        let output = map.insert(12..22, 'x');

        assert!(output.eq([
            ((12..14).into_interval(), 'a'),
            ((16..18).into_interval(), 'b'),
            ((20..22).into_interval(), 'c'),
        ]));
        assert!(map.dbg_entries().eq([
            ((10..12).into_interval(), 'a'),
            ((12..22).into_interval(), 'x'),
            ((22..24).into_interval(), 'c'),
        ]))
    }
}
