use enum_indexed::indexed_struct::{EnumIndexed as _, IndexedStruct};

#[derive(Debug, Clone, Copy, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn map() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.map(|n| n.to_string());

    assert_eq!(
        strings,
        MyEnumIndexed {
            a: "1".to_string(),
            b: "2".to_string(),
            c: "3".to_string()
        }
    );
}

#[test]
fn map_enumerated() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.map_enumerated(|e, n| format!("{e:?}{n}"));

    assert_eq!(
        strings,
        MyEnumIndexed {
            a: "A1".to_string(),
            b: "B2".to_string(),
            c: "C3".to_string()
        }
    );
}

#[test]
fn try_map_ok() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.try_map(|n| {
        if n == 0 {
            Err("failed")
        } else {
            Ok(n.to_string())
        }
    });

    assert_eq!(
        strings,
        Ok(MyEnumIndexed {
            a: "1".to_string(),
            b: "2".to_string(),
            c: "3".to_string()
        })
    );
}

#[test]
fn try_map_err() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.try_map(|n| {
        if n == 2 {
            Err("failed")
        } else {
            Ok(n.to_string())
        }
    });

    assert_eq!(strings, Err("failed"));
}

#[test]
fn try_map_enumerated_ok() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.try_map_enumerated(|e, n| {
        if n == 0 {
            Err("failed")
        } else {
            Ok(format!("{e:?}{n}"))
        }
    });

    assert_eq!(
        strings,
        Ok(MyEnumIndexed {
            a: "A1".to_string(),
            b: "B2".to_string(),
            c: "C3".to_string()
        })
    );
}

#[test]
fn try_map_enumerated_err() {
    let numbers = MyEnumIndexed { a: 1, b: 2, c: 3 };

    let strings = numbers.try_map_enumerated(|e, n| {
        if n == 2 {
            Err("failed")
        } else {
            Ok(format!("{e:?}{n}"))
        }
    });

    assert_eq!(strings, Err("failed"));
}
