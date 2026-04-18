use serde_json::json;
use test_case::test_case;

use super::*;
use crate::comparison::{OneSideOnlyIndexes, Scalar, ScalarPair, Side};

macro_rules! map {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        [ $( (($key).to_string(), $value), )* ]
        .into_iter()
        .collect()
    };
}

#[test_case(
    json!(1), json!("b")
    => Comparison::DifferentTypes(json!(1), json!("b"));
    "different types"
)]
#[test_case(
    json!(null), json!(null)
    => Comparison::Scalars(ScalarsComparison::Same(Scalar::Null));
    "nulls"
)]
#[test_case(
    json!(true), json!(true)
    => Comparison::Scalars(ScalarsComparison::Same(Scalar::Bool(true)));
    "bools: equal"
)]
#[test_case(
    json!(true), json!(false)
    => Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Bools(true, false)));
    "bools: different"
)]
#[test_case(
    json!(7), json!(7)
    => Comparison::Scalars(ScalarsComparison::Same(7.into()));
    "numbers: equal"
)]
#[test_case(
    json!(3), json!(4)
    => Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Numbers(3.into(), 4.into())));
    "numbers: different"
)]
#[test_case(
    json!("a"), json!("a")
    => Comparison::Scalars(ScalarsComparison::Same("a".into()));
    "strings: equal"
)]
#[test_case(
    json!("a"), json!("b")
    => Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Strings("a".into(), "b".into())));
    "strings: different"
)]
#[test_case(
    json!([true, 1, "a"]), json!([true, 1, "a"])
    => Comparison::Arrays(ArraysComparison {
        common_indexes: vec![
            Comparison::Scalars(ScalarsComparison::Same(true.into())),
            Comparison::Scalars(ScalarsComparison::Same(1.into())),
            Comparison::Scalars(ScalarsComparison::Same("a".into())),
        ],
        one_side_only_indexes: None,
    });
    "arrays: equal"
)]
#[test_case(
    json!([true, 1, "a"]), json!([true, 2, "a"])
    => Comparison::Arrays(ArraysComparison {
        common_indexes: vec![
            Comparison::Scalars(ScalarsComparison::Same(true.into())),
            Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Numbers(1.into(), 2.into()))),
            Comparison::Scalars(ScalarsComparison::Same("a".into())),
        ],
        one_side_only_indexes: None,
    });
    "arrays: different, same length"
)]
#[test_case(
    json!([true, 1, "a", "b", "c"]), json!([true, 1, "a"])
    => Comparison::Arrays(ArraysComparison {
        common_indexes: vec![
            Comparison::Scalars(ScalarsComparison::Same(true.into())),
            Comparison::Scalars(ScalarsComparison::Same(1.into())),
            Comparison::Scalars(ScalarsComparison::Same("a".into())),
        ],
        one_side_only_indexes: Some(OneSideOnlyIndexes {
            values: vec![json!("b"), json!("c")],
            side: Side::Left,
        })
    });
    "arrays: left is longer"
)]
#[test_case(
    json!([true, 1, "a"]), json!([true, 1, "a", "b", "c"])
    => Comparison::Arrays(ArraysComparison {
        common_indexes: vec![
            Comparison::Scalars(ScalarsComparison::Same(true.into())),
            Comparison::Scalars(ScalarsComparison::Same(1.into())),
            Comparison::Scalars(ScalarsComparison::Same("a".into())),
        ],
        one_side_only_indexes: Some(OneSideOnlyIndexes {
            values: vec![json!("b"), json!("c")],
            side: Side::Right,
        })
    });
    "arrays: right is longer"
)]
#[test_case(
    json!({
        "a": true,
        "b": 1,
        "c": "x",
        "d": false,
    }),
    json!({
        "b": 1,
        "c": "y",
        "d": "z",
        "e": 7,
    })
    => Comparison::Objects(ObjectsComparison {
        common_entries: map! {
            "b" => Comparison::Scalars(ScalarsComparison::Same(1.into())),
            "c" => Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Strings("x".into(), "y".into()))),
            "d" => Comparison::DifferentTypes(json!(false), json!("z")),
        },
        left_only_entries: map! { "a" => Value::from(true) },
        right_only_entries: map! { "e" => Value::from(7) },
    });
    "objects"
)]
#[test_case(
    json!([
        true,
        {
            "a": "z",
            "b": [
                false,
                null,
            ],
            "c": 2,
        },
        1,
    ]),
    json!([
        true,
        {
            "a": "z",
            "b": [
                false,
                null,
            ],
            "c": 2,
        },
        1,
    ])
    => Comparison::Arrays(ArraysComparison {
        common_indexes: vec![
            Comparison::Scalars(ScalarsComparison::Same(true.into())),
            Comparison::Objects(ObjectsComparison {
                common_entries: map! {
                    "a" => Comparison::Scalars(ScalarsComparison::Same("z".into())),
                    "b" => Comparison::Arrays(ArraysComparison {
                        common_indexes: vec![
                            Comparison::Scalars(ScalarsComparison::Same(false.into())),
                            Comparison::Scalars(ScalarsComparison::Same(Scalar::Null)),
                        ],
                        one_side_only_indexes: None,
                    }),
                    "c" => Comparison::Scalars(ScalarsComparison::Same(2.into())),
                },
                left_only_entries: map! {},
                right_only_entries: map! {},
            }),
            Comparison::Scalars(ScalarsComparison::Same(1.into())),
        ],
        one_side_only_indexes: None
    });
    "array inside object inside array"
)]
fn compare_test(left: Value, right: Value) -> Comparison {
    compare(left, right)
}
