use enum_indexed::indexed_struct::IndexedStruct;

#[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
    #[indexed_struct(skip)]
    D,
}

#[test]
fn index() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    assert_eq!(numbers[MyEnum::A], 1);
    assert_eq!(numbers[MyEnum::B], 2);
    assert_eq!(numbers[MyEnum::C], 3);
}

#[test]
#[should_panic(expected = "variant is not supported as an index")]
fn skipped_variant_index() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let _ = numbers[MyEnum::D];
}

#[test]
fn index_ref() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    assert_eq!(&numbers[MyEnum::A], &1);
    assert_eq!(&numbers[MyEnum::B], &2);
    assert_eq!(&numbers[MyEnum::C], &3);
}

#[test]
#[should_panic(expected = "variant is not supported as an index")]
fn skipped_variant_index_ref() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let _ = &numbers[MyEnum::D];
}

#[test]
fn index_mut() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    assert_eq!(&mut numbers[MyEnum::A], &mut 1);
    assert_eq!(&mut numbers[MyEnum::B], &mut 2);
    assert_eq!(&mut numbers[MyEnum::C], &mut 3);
}

#[test]
#[should_panic(expected = "variant is not supported as an index")]
fn skipped_variant_index_mut() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let _ = &mut numbers[MyEnum::D];
}
