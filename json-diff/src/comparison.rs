use std::collections::BTreeMap;

use serde_json::{Map, Number, Value};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Comparison {
    Scalars(ScalarsComparison),
    Arrays(ArraysComparison),
    Objects(ObjectsComparison),
    DifferentTypes(Value, Value),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScalarsComparison {
    Same(Scalar),
    Different(ScalarPair),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Scalar {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
}
impl From<bool> for Scalar {
    fn from(value: bool) -> Self {
        Scalar::Bool(value)
    }
}
impl From<Number> for Scalar {
    fn from(value: Number) -> Self {
        Scalar::Number(value)
    }
}
impl From<u32> for Scalar {
    fn from(value: u32) -> Self {
        Scalar::Number(value.into())
    }
}
impl From<String> for Scalar {
    fn from(value: String) -> Self {
        Scalar::String(value)
    }
}
impl From<&str> for Scalar {
    fn from(value: &str) -> Self {
        Scalar::String(value.into())
    }
}
impl From<Scalar> for Value {
    fn from(value: Scalar) -> Self {
        match value {
            Scalar::Null => Value::Null,
            Scalar::Bool(bool) => Value::Bool(bool),
            Scalar::Number(number) => Value::Number(number),
            Scalar::String(string) => Value::String(string),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ScalarPair {
    Bools(bool, bool),
    Numbers(Number, Number),
    Strings(String, String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ArraysComparison {
    pub(crate) common_indexes: Vec<Comparison>,
    pub(crate) one_side_only_indexes: Option<OneSideOnlyIndexes>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OneSideOnlyIndexes {
    pub(crate) values: Vec<Value>,
    pub(crate) side: Side,
}

#[derive(Debug, Clone, PartialEq, Eq)]
#[expect(clippy::struct_field_names)]
pub(crate) struct ObjectsComparison {
    pub(crate) common_entries: BTreeMap<String, Comparison>,
    pub(crate) left_only_entries: Map<String, Value>,
    pub(crate) right_only_entries: Map<String, Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}
