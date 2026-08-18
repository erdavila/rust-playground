use enum_indexed::indexed_struct::{EnumIndexed as _, IndexedStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn into_array() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let array = numbers.into_array();

    assert_eq!(array, [1, 2, 3]);
}

#[test]
fn into_array_enumerated() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let array = numbers.into_array_enumerated();

    assert_eq!(array, [(MyEnum::A, 1), (MyEnum::B, 2), (MyEnum::C, 3)]);
}
