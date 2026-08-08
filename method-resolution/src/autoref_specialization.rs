#![expect(
    clippy::needless_borrow,
    clippy::needless_raw_string_hashes,
    clippy::unused_self
)]

// Reference: https://github.com/dtolnay/case-studies/tree/master/autoref-specialization

mod original;

pub(crate) mod by_reference {
    use std::fmt::{Debug, Display};

    trait ByReference {
        fn by_reference(&self) -> String;
    }

    // Blanket impl.
    impl<T: Display + Debug> ByReference for &T {
        fn by_reference(&self) -> String {
            format!("<&T as ByReference>::by_reference({self:?})")
        }
    }

    // "Specialized" impl.
    impl ByReference for String {
        fn by_reference(&self) -> String {
            format!("<String as ByReference>::by_reference({self:?})")
        }
    }

    macro_rules! by_references {
        ($($e:expr),*) => {
            [
                $(
                    (&$e).by_reference()
                ),*
            ]
        };
    }

    pub(super) fn demo() {
        let owned_string = "owned_string".to_string();

        let strings = by_references![1, "&str", owned_string];
        assert_eq!(
            strings,
            [
                r#"<&T as ByReference>::by_reference(1)"#,
                r#"<&T as ByReference>::by_reference("&str")"#,
                r#"<String as ByReference>::by_reference("owned_string")"#,
            ],
        );

        // The call:
        (&1).by_reference();
        // Is translated to:
        <&i32 as ByReference>::by_reference(&&1);

        // The call:
        (&"&str").by_reference();
        // Is translated to:
        <&&str as ByReference>::by_reference(&&"&str");

        // The call:
        (&owned_string).by_reference();
        // Is translated to:
        <String as ByReference>::by_reference(&owned_string);
    }
}

pub(crate) mod by_value {
    use std::fmt::{Debug, Display};
    use std::marker::PhantomData;

    trait ByValue<T> {
        // `self` is not meant to be used.
        fn by_value(self, value: T) -> String;
    }

    trait Kind<T> {
        type ByValue: ByValue<T>;

        fn kind(&self) -> Self::ByValue;
    }

    // Blanket impl.
    impl<T: Display + Debug + 'static> Kind<T> for &T {
        type ByValue = DisplayByValue<T>;

        fn kind(&self) -> Self::ByValue {
            DisplayByValue(PhantomData)
        }
    }

    struct DisplayByValue<T>(PhantomData<T>);

    impl<U: Display + Debug> ByValue<U> for DisplayByValue<U> {
        fn by_value(self, value: U) -> String {
            format!("DisplayByValue::by_value({value:?})")
        }
    }

    // "Specialized" impl.
    impl Kind<String> for String {
        type ByValue = StringByValue;

        fn kind(&self) -> Self::ByValue {
            StringByValue
        }
    }

    struct StringByValue;

    impl ByValue<String> for StringByValue {
        fn by_value(self, value: String) -> String {
            format!("StringByValue::by_value({value:?})")
        }
    }

    macro_rules! _by_values {
        ($($e:expr),*) => {
            [
                $(
                    {
                        let e = $e;
                        (&e).kind().by_value(e)
                    }
                ),*
            ]
        };
    }

    pub(super) fn demo() {
        let owned_string0 = "owned_string".to_string();
        let owned_string1 = owned_string0.clone();
        let owned_string2 = owned_string0.clone();

        let strings = _by_values![1, "&str", owned_string0];
        assert_eq!(
            strings,
            [
                r#"DisplayByValue::by_value(1)"#,
                r#"DisplayByValue::by_value("&str")"#,
                r#"StringByValue::by_value("owned_string")"#,
            ],
        );

        // The call:
        (&1).kind().by_value(1);
        // Is translated to:
        let k = <&i32 as Kind<i32>>::kind(&&1);
        <DisplayByValue<i32> as ByValue<i32>>::by_value(k, 1);

        // The call:
        (&"&str").kind().by_value("&str");
        // Is translated to:
        let k = <&&str as Kind<&str>>::kind(&&"&str");
        <DisplayByValue<&str> as ByValue<&str>>::by_value(k, "&str");

        // The call:
        (&owned_string1).kind().by_value(owned_string1);
        // Is translated to:
        let k = <String as Kind<String>>::kind(&owned_string2);
        <StringByValue as ByValue<String>>::by_value(k, owned_string2);
    }
}

pub(super) fn demo() {
    by_reference::demo();
    by_value::demo();
    original::demo();
}
