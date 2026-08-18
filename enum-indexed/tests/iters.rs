use enum_indexed::indexed_struct::{EnumIndexed as _, IndexedStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn iter_next() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some((MyEnum::A, &1)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some((MyEnum::B, &2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((MyEnum::C, &3)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_next_back() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some((MyEnum::C, &3)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some((MyEnum::B, &2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((MyEnum::A, &1)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn iter_mut_next() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.iter_mut();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some((MyEnum::A, &mut 1)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some((MyEnum::B, &mut 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((MyEnum::C, &mut 3)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn iter_mut_next_back() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.iter_mut();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some((MyEnum::C, &mut 3)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some((MyEnum::B, &mut 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((MyEnum::A, &mut 1)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn into_iter_next() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some((MyEnum::A, 1)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some((MyEnum::B, 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((MyEnum::C, 3)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_next_back() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some((MyEnum::C, 3)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some((MyEnum::B, 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((MyEnum::A, 1)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn into_iter_ref_next() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = (&numbers).into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some((MyEnum::A, &1)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some((MyEnum::B, &2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((MyEnum::C, &3)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_ref_next_back() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = (&numbers).into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some((MyEnum::C, &3)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some((MyEnum::B, &2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((MyEnum::A, &1)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn into_iter_mut_next() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = (&mut numbers).into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some((MyEnum::A, &mut 1)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some((MyEnum::B, &mut 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some((MyEnum::C, &mut 3)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_iter_mut_next_back() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = (&mut numbers).into_iter();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some((MyEnum::C, &mut 3)));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some((MyEnum::B, &mut 2)));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some((MyEnum::A, &mut 1)));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn values_next() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.values();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&3));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn values_next_back() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.values();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(&3));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some(&2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some(&1));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn values_mut_next() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.values_mut();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(&mut 1));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(&mut 2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(&mut 3));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn values_mut_next_back() {
    let mut numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.values_mut();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(&mut 3));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some(&mut 2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some(&mut 1));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}

#[test]
fn into_values_next() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.into_values();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next(), None);
}

#[test]
fn into_values_next_back() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let mut iter = numbers.into_values();

    assert_eq!(iter.size_hint(), (3, Some(3)));
    assert_eq!(iter.next_back(), Some(3));
    assert_eq!(iter.size_hint(), (2, Some(2)));
    assert_eq!(iter.next_back(), Some(2));
    assert_eq!(iter.size_hint(), (1, Some(1)));
    assert_eq!(iter.next_back(), Some(1));
    assert_eq!(iter.size_hint(), (0, Some(0)));
    assert_eq!(iter.next_back(), None);
}
