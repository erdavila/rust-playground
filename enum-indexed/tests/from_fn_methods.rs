use enum_indexed::indexed_struct::{EnumIndexed as _, IndexedStruct};

#[derive(Debug, Clone, Copy, PartialEq, Eq, IndexedStruct)]
enum MyEnum {
    A,
    B,
    C,
}

#[test]
fn from_fn() {
    let values = MyEnumIndexed::from_fn(|e| format!("{e:?}"));

    assert_eq!(
        values,
        MyEnumIndexed {
            a: "A".to_string(),
            b: "B".to_string(),
            c: "C".to_string()
        }
    );
}

#[test]
fn try_from_fn_ok() {
    let values = MyEnumIndexed::try_from_fn(|e| {
        if false {
            Err("failed")
        } else {
            Ok(format!("{e:?}"))
        }
    });

    assert_eq!(
        values,
        Ok(MyEnumIndexed {
            a: "A".to_string(),
            b: "B".to_string(),
            c: "C".to_string()
        })
    );
}

#[test]
fn try_from_fn_err() {
    let values = MyEnumIndexed::try_from_fn(|e| {
        if e == MyEnum::B {
            Err("failed")
        } else {
            Ok(format!("{e:?}"))
        }
    });

    assert_eq!(values, Err("failed"));
}
