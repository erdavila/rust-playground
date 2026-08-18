use enum_indexed::indexed_struct::{EnumIndexed, IndexedStruct};

#[derive(Debug, Copy, Clone, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn as_ref() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let as_ref = numbers.as_ref();

    assert_eq!(as_ref.a, &1);
    assert_eq!(as_ref.b, &2);
    assert_eq!(as_ref.c, &3);
}

#[test]
fn as_mut() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let as_mut = numbers.as_mut();
    *as_mut.a *= 2;
    *as_mut.b *= 2;
    *as_mut.c *= 2;

    assert_eq!(numbers.a, 2);
    assert_eq!(numbers.b, 4);
    assert_eq!(numbers.c, 6);
}
