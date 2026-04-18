use std::collections::BTreeMap;
use std::iter;

use serde_json::{Map, Value};

use crate::comparison::{
    ArraysComparison, Comparison, ObjectsComparison, OneSideOnlyIndexes, Scalar, ScalarPair,
    ScalarsComparison, Side,
};

pub(crate) fn compare(left: Value, right: Value) -> Comparison {
    match (left, right) {
        (Value::Null, Value::Null) => Comparison::Scalars(ScalarsComparison::Same(Scalar::Null)),
        (Value::Bool(left), Value::Bool(right)) => compare_scalars(left, right, ScalarPair::Bools),
        (Value::Number(left), Value::Number(right)) => {
            compare_scalars(left, right, ScalarPair::Numbers)
        }
        (Value::String(left), Value::String(right)) => {
            compare_scalars(left, right, ScalarPair::Strings)
        }
        (Value::Array(left), Value::Array(right)) => compare_arrays(left, right),
        (Value::Object(left), Value::Object(right)) => compare_objects(left, right),
        (left, right) => Comparison::DifferentTypes(left, right),
    }
}

fn compare_scalars<T: Into<Scalar> + PartialEq>(
    left: T,
    right: T,
    to_pair: fn(T, T) -> ScalarPair,
) -> Comparison {
    if left == right {
        Comparison::Scalars(ScalarsComparison::Same(left.into()))
    } else {
        Comparison::Scalars(ScalarsComparison::Different(to_pair(left, right)))
    }
}

fn compare_arrays(left: Vec<Value>, right: Vec<Value>) -> Comparison {
    let mut common_indexes = Vec::with_capacity(left.len().min(right.len()));

    let mut left_iter = left.into_iter();
    let mut right_iter = right.into_iter();

    let one_side_only_indexes = loop {
        fn build_one_side_only_indexes(
            first: Value,
            remaining: impl IntoIterator<Item = Value>,
            side: Side,
        ) -> OneSideOnlyIndexes {
            OneSideOnlyIndexes {
                values: iter::once(first).chain(remaining).collect(),
                side,
            }
        }

        match (left_iter.next(), right_iter.next()) {
            (Some(left), Some(right)) => common_indexes.push(compare(left, right)),
            (Some(left), None) => {
                break Some(build_one_side_only_indexes(left, left_iter, Side::Left));
            }
            (None, Some(right)) => {
                break Some(build_one_side_only_indexes(right, right_iter, Side::Right));
            }
            (None, None) => break None,
        }
    };

    Comparison::Arrays(ArraysComparison {
        common_indexes,
        one_side_only_indexes,
    })
}

fn compare_objects(left: Map<String, Value>, mut right: Map<String, Value>) -> Comparison {
    let mut common_entries = BTreeMap::new();
    let mut left_only_entries = Map::new();

    for (key, left_val) in left {
        match right.remove(&key) {
            Some(right_val) => {
                common_entries.insert(key, compare(left_val, right_val));
            }
            None => {
                left_only_entries.insert(key, left_val);
            }
        }
    }

    let right_only_entries = right;

    Comparison::Objects(ObjectsComparison {
        common_entries,
        left_only_entries,
        right_only_entries,
    })
}

#[cfg(test)]
mod tests;
