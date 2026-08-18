use enum_indexed::indexed_struct::{EnumIndexed, IndexedStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
    #[indexed_struct(skip)]
    D,
}

#[test]
fn swap() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    numbers.swap(MyEnum::A, MyEnum::B);

    assert_eq!(numbers, MyEnumIndexed { a: 2, b: 1, c: 3 });

    numbers.swap(MyEnum::A, MyEnum::C);

    assert_eq!(numbers, MyEnumIndexed { a: 3, b: 1, c: 2 });

    numbers.swap(MyEnum::B, MyEnum::C);

    assert_eq!(numbers, MyEnumIndexed { a: 3, b: 2, c: 1 });
}

#[test]
fn same_index() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    numbers.swap(MyEnum::B, MyEnum::B);

    assert_eq!(numbers, MyEnumIndexed { a: 1, b: 2, c: 3 });
}

#[test]
#[should_panic(expected = "variant is not supported as an index")]
fn skipped_variant() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    numbers.swap(MyEnum::A, MyEnum::D);
}
