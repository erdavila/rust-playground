use std::collections::BTreeMap;

use serde_json::{Map, json};
use test_case::test_case;

use super::*;
use crate::compare::compare;
use crate::comparison::{
    ArraysComparison, Comparison, ObjectsComparison, OneSideOnlyIndexes, Scalar, ScalarPair,
    ScalarsComparison,
};
use crate::diff::tests::{TestPutToken, tokens};
use crate::token::Token;

fn inline_diff_tokens(comparison: Comparison) -> Vec<Token> {
    let mut put_token = TestPutToken::new();
    tokenize(&mut put_token, comparison).unwrap();

    put_token.tokens
}

#[test_case(Scalar::Null => tokens![Null, NewLine]; "null")]
#[test_case(true => tokens![Scalar(true), NewLine]; "bool")]
#[test_case(7 => tokens![Scalar(7), NewLine]; "number")]
#[test_case("abc" => tokens![Scalar("abc"), NewLine]; "string")]
fn same_scalars(scalar: impl Into<Scalar>) -> Vec<Token> {
    let comparison = Comparison::Scalars(ScalarsComparison::Same(scalar.into()));
    inline_diff_tokens(comparison)
}

#[test_case(
    ScalarPair::Bools(false, true)
    => tokens![
        Left { Scalar(false) },
        Right { Scalar(true) },
        NewLine,
    ];
    "bools"
)]
#[test_case(
    ScalarPair::Numbers(3.into(), 4.into())
    => tokens![
        Left { Scalar(3) },
        Right { Scalar(4) },
        NewLine,
    ];
    "numbers"
)]
#[test_case(
    ScalarPair::Strings("a".into(), "b".into())
    => tokens![
        Left { Scalar("a") },
        Right { Scalar("b") },
        NewLine,
    ];
    "strings"
)]
fn different_scalars(scalars: ScalarPair) -> Vec<Token> {
    let comparison = Comparison::Scalars(ScalarsComparison::Different(scalars));
    inline_diff_tokens(comparison)
}

#[test_case(
    json!(3), json!("a")
    => tokens![
        Left { Scalar(3) },
        Right { Scalar("a") },
        NewLine,
    ];
    "number vs string"
)]
#[test_case(
    json!(null), json!(false)
    => tokens![
        Left { Null },
        Right { Scalar(false) },
        NewLine,
    ];
    "null vs bool"
)]
#[test_case(
    json!([]), json!({})
    => tokens![
        Left { ArrayBegin, ArrayEnd },
        Right { ObjectBegin, ObjectEnd },
        NewLine,
    ];
    "empty array vs empty object"
)]
#[test_case(
    json!([true, 1, "x",]), json!({ "a": true, "b": 1, "c": "x" })
    => tokens![
        Left {
            ArrayBegin, NewLine,
            Indent, Scalar(true), Comma, NewLine,
            Indent, Scalar(1), Comma, NewLine,
            Indent, Scalar("x"), NewLine,
            ArrayEnd,
        },
        Right {
            ObjectBegin, NewLine,
            Indent, Key("a"), Scalar(true), Comma, NewLine,
            Indent, Key("b"), Scalar(1), Comma, NewLine,
            Indent, Key("c"), Scalar("x"), NewLine,
            ObjectEnd,
        },
        NewLine,
    ];
    "non-empty array vs non-empty object"
)]
fn different_types(left: Value, right: Value) -> Vec<Token> {
    let comparison = Comparison::DifferentTypes(left, right);
    inline_diff_tokens(comparison)
}

#[test_case(
    // common
    Vec::new(),
    // one side only
    None
    => tokens![ArrayBegin, ArrayEnd, NewLine];
    "both empty"
)]
#[test_case(
    // common
    Vec::new(),
    // one side only
    Some(OneSideOnlyIndexes {
        values: vec![json!(true), json!(1), json!("a")],
        side: Side::Left,
    })
    => tokens![
        Left {
            ArrayBegin, NewLine,
            Indent, Scalar(true), Comma, NewLine,
            Indent, Scalar(1), Comma, NewLine,
            Indent, Scalar("a"), NewLine,
            ArrayEnd,
        },
        Right {
            ArrayBegin, ArrayEnd,
        },
        NewLine,
    ];
    "non-empty vs empty"
)]
#[test_case(
    // common
    Vec::new(),
    // one side only
    Some(OneSideOnlyIndexes {
        values: vec![json!(true), json!(1), json!("a")],
        side: Side::Right,
    })
    => tokens![
        Left {
            ArrayBegin, ArrayEnd,
        },
        Right {
            ArrayBegin, NewLine,
            Indent, Scalar(true), Comma, NewLine,
            Indent, Scalar(1), Comma, NewLine,
            Indent, Scalar("a"), NewLine,
            ArrayEnd,
        },
        NewLine,
    ];
    "empty vs non-empty"
)]
#[test_case(
    // common
    vec![
        Comparison::Scalars(ScalarsComparison::Same(true.into())),
        Comparison::Scalars(ScalarsComparison::Same(1.into())),
        Comparison::Scalars(ScalarsComparison::Same("a".into())),
    ],
    // one side only
    None
    => tokens![
        ArrayBegin, NewLine,
        Indent, Scalar(true), Comma, NewLine,
        Indent, Scalar(1), Comma, NewLine,
        Indent, Scalar("a"), NewLine,
        ArrayEnd, NewLine,
    ];
    "equal non-empty arrays"
)]
#[test_case(
    // common
    vec![
        Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Bools(false, true))),
        Comparison::DifferentTypes(json!(1), json!(null)),
        Comparison::Scalars(ScalarsComparison::Same("a".into())),
    ],
    // one side only
    None
    => tokens![
        ArrayBegin, NewLine,
        Indent,
            Left { Scalar(false) },
            Right { Scalar(true) },
        Comma, NewLine,
        Indent,
            Left { Scalar(1) },
            Right { Null },
        Comma, NewLine,
        Indent, Scalar("a"), NewLine,
        ArrayEnd, NewLine,
    ];
    "different same-length arrays"
)]
#[test_case(
    // common
    vec![
        Comparison::Scalars(ScalarsComparison::Same(Scalar::Null)),
        Comparison::Scalars(ScalarsComparison::Same(true.into())),
        Comparison::Scalars(ScalarsComparison::Same(1.into())),
    ],
    // one side only
    Some(OneSideOnlyIndexes { values: vec![json!("a"), json!("b")], side: Side::Left })
    => tokens![
        ArrayBegin, NewLine,
        Indent, Null, Comma, NewLine,
        Indent, Scalar(true), Comma, NewLine,
        Indent, Scalar(1),
            Left { Comma },
        NewLine,
        Indent,
            Left { Scalar("a"), Comma },
        NewLine,
        Indent,
            Left { Scalar("b") },
        NewLine,
        ArrayEnd, NewLine,
    ];
    "non-empty arrays with elements only in left"
)]
#[test_case(
    // common
    vec![
        Comparison::Scalars(ScalarsComparison::Same(Scalar::Null)),
        Comparison::Scalars(ScalarsComparison::Same(true.into())),
        Comparison::Scalars(ScalarsComparison::Same(1.into())),
    ],
    // one side only
    Some(OneSideOnlyIndexes { values: vec![json!("a"), json!("b")], side: Side::Right })
    => tokens![
        ArrayBegin, NewLine,
        Indent, Null, Comma, NewLine,
        Indent, Scalar(true), Comma, NewLine,
        Indent, Scalar(1),
            Right { Comma },
        NewLine,
        Indent,
            Right { Scalar("a"), Comma },
        NewLine,
        Indent,
            Right { Scalar("b") },
        NewLine,
        ArrayEnd, NewLine,
    ];
    "non-empty arrays with elements only in right"
)]
fn arrays(
    common_indexes: Vec<Comparison>,
    one_side_only_indexes: Option<OneSideOnlyIndexes>,
) -> Vec<Token> {
    let comparison = Comparison::Arrays(ArraysComparison {
        common_indexes,
        one_side_only_indexes,
    });
    inline_diff_tokens(comparison)
}

#[test_case(
    // common
    BTreeMap::new(),
    // left-only
    Map::new(),
    // right-only
    Map::new()
    => tokens![ObjectBegin, ObjectEnd, NewLine];
    "both empty"
)]
#[test_case(
    //common
    BTreeMap::new(),
    // left-only
    Map::from_iter([
        ("a".into(), json!(true)),
        ("b".into(), json!(1)),
        ("c".into(), json!("x")),
    ]),
    // right-only
    Map::new()
    => tokens![
        Left {
            ObjectBegin, NewLine,
            Indent, Key("a"), Scalar(true), Comma, NewLine,
            Indent, Key("b"), Scalar(1), Comma, NewLine,
            Indent, Key("c"), Scalar("x"), NewLine,
            ObjectEnd,
        },
        Right {
            ObjectBegin, ObjectEnd,
        },
        NewLine,
    ];
    "non-empty vs empty"
)]
#[test_case(
    //common
    BTreeMap::new(),
    // left-only
    Map::new(),
    // right-only
    Map::from_iter([
        ("a".into(), json!(true)),
        ("b".into(), json!(1)),
        ("c".into(), json!("x")),
    ])
    => tokens![
        Left {
            ObjectBegin, ObjectEnd,
        },
        Right {
            ObjectBegin, NewLine,
            Indent, Key("a"), Scalar(true), Comma, NewLine,
            Indent, Key("b"), Scalar(1), Comma, NewLine,
            Indent, Key("c"), Scalar("x"), NewLine,
            ObjectEnd,
        },
        NewLine,
    ];
    "empty vs non-empty"
)]
#[test_case(
    //common
    BTreeMap::from_iter([
        ("a".into(), Comparison::Scalars(ScalarsComparison::Same(true.into()))),
        ("b".into(), Comparison::Scalars(ScalarsComparison::Same(1.into()))),
        ("c".into(), Comparison::Scalars(ScalarsComparison::Same("x".into()))),
    ]),
    // left-only
    Map::new(),
    // right-only
    Map::new()
    => tokens![
        ObjectBegin, NewLine,
        Indent, Key("a"), Scalar(true), Comma, NewLine,
        Indent, Key("b"), Scalar(1), Comma, NewLine,
        Indent, Key("c"), Scalar("x"), NewLine,
        ObjectEnd, NewLine,
    ];
    "equal non-empty objects"
)]
#[test_case(
    //common
    BTreeMap::from_iter([
        ("a".into(), Comparison::Scalars(ScalarsComparison::Different(ScalarPair::Bools(false, true)))),
        ("b".into(), Comparison::DifferentTypes(json!(1), json!(null))),
        ("c".into(), Comparison::Scalars(ScalarsComparison::Same("x".into()))),
    ]),
    // left-only
    Map::new(),
    // right-only
    Map::new()
    => tokens![
        ObjectBegin, NewLine,
        Indent, Key("a"),
            Left { Scalar(false) },
            Right { Scalar(true) },
        Comma, NewLine,
        Indent, Key("b"),
            Left { Scalar(1) },
            Right { Null },
        Comma, NewLine,
        Indent, Key("c"), Scalar("x"), NewLine,
        ObjectEnd, NewLine,
    ];
    "different same-keys objects"
)]
#[test_case(
    //common
    BTreeMap::from_iter([
        ("m".into(), Comparison::Scalars(ScalarsComparison::Same(Scalar::Null))),
        ("n".into(), Comparison::Scalars(ScalarsComparison::Same(true.into()))),
        ("o".into(), Comparison::Scalars(ScalarsComparison::Same(1.into()))),
    ]),
    // left-only
    Map::from_iter([
        ("a".into(), json!(3))
    ]),
    // right-only
    Map::from_iter([
        ("z".into(), json!(4))
    ])
    => tokens![
        ObjectBegin, NewLine,
        Indent,
            Left { Key("a"), Scalar(3), Comma,  },
        NewLine,
        Indent, Key("m"), Null, Comma, NewLine,
        Indent, Key("n"), Scalar(true), Comma, NewLine,
        Indent, Key("o"), Scalar(1),
            Right { Comma },
        NewLine,
        Indent,
            Right { Key("z"), Scalar(4) },
        NewLine,
        ObjectEnd, NewLine,
    ];
    "non-empty objects with elements only in the sides"
)]
fn objects(
    common_entries: BTreeMap<String, Comparison>,
    left_only_entries: Map<String, Value>,
    right_only_entries: Map<String, Value>,
) -> Vec<Token> {
    let comparison = Comparison::Objects(ObjectsComparison {
        common_entries,
        left_only_entries,
        right_only_entries,
    });
    inline_diff_tokens(comparison)
}

#[test]
fn compare_and_tokenize() {
    let left = json!({
        "1-same": true,
        "2-different_scalars": 3,
        "3-different_types": null,
        "4-only_in_left": [],
        "5-object": {
            "a": true,
            "b": 1,
            "c": "x",
        },
    });
    let right = json!({
        "1-same": true,
        "2-different_scalars": 4,
        "3-different_types": "x",
        "5-object": {
            "a": true,
            "b": 1,
            "c": "x",
        },
        "6-only_in_right": "y",
    });

    let comparison = compare(left, right);
    let tokens = inline_diff_tokens(comparison);

    assert_eq!(
        tokens,
        tokens![
            ObjectBegin, NewLine,
            Indent, Key("1-same"), Scalar(true), Comma, NewLine,
            Indent, Key("2-different_scalars"),
                Left { Scalar(3) },
                Right { Scalar(4) },
            Comma, NewLine,
            Indent, Key("3-different_types"),
                Left { Null },
                Right { Scalar("x") },
            Comma, NewLine,
            Indent,
                Left { Key("4-only_in_left"), ArrayBegin, ArrayEnd, Comma},
            NewLine,
            Indent, Key("5-object"), ObjectBegin, NewLine,
            Indent, Indent, Key("a"), Scalar(true), Comma, NewLine,
            Indent, Indent, Key("b"), Scalar(1), Comma, NewLine,
            Indent, Indent, Key("c"), Scalar("x"), NewLine,
            Indent, ObjectEnd,
                Right { Comma },
            NewLine,
            Indent,
                Right { Key("6-only_in_right"), Scalar("y") },
            NewLine,
            ObjectEnd, NewLine,
        ]
    );
}
