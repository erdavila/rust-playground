#![expect(
    clippy::new_ret_no_self,
    clippy::uninlined_format_args,
    clippy::wrong_self_convention,
    unused_variables
)]

/*
    The call `(&1).my_to_string()` is resolved like this:
        1. By value: <&i32>::my_to_string(&i32)
            Not found.
        2. Autoref: <&i32>::my_to_string(&<&i32>)
            Found <&i32 as DisplayToSTring>::my_to_string(&self) -> String
            Executes:
                let receiver: &i32 = &1;
                <&i32 as DisplayToSTring>::my_to_string(&receiver)

    The call `(&"&str").my_to_string()` is resolved like this:
        1. By value: <&&str>::my_to_string(&&str)
            Not found.
        2. Autoref: <&&str>::my_to_string(&<&&str>)
            Found <&&str as DisplayToSTring>::my_to_string(&self) -> String
            Executes:
                let receiver: &&str = &"&str";
                <&&str as DisplayToSTring>::my_to_string(&receiver)

    The call `(&owned_string).my_to_string()` is resolved like this:
        1. By value: <&String>::my_to_string(&String)
            Not found.
        2. Autoref: <&String>::my_to_string(&<&String>)
            Not found.
        3. Deref: <&String as Deref>::Target = String
            1. By value: String::my_to_string(String)
                Not found.
            2. Autoref: String::my_to_string(&String)
                Found <String as StringToString>::my_to_string(&self) -> String
                Executes:
                    let receiver: &String = &owned_string;
                    let string_ref: &String = <&String as Deref>::deref(&receiver);
                    <String as StringToString>::my_to_string(&*string_ref)
*/

mod by_reference {
    use std::fmt::{Display, Write};

    pub trait DisplayToString {
        fn my_to_string(&self) -> String;
    }

    // General impl that applies to any T with a Display impl.
    //
    // Note that the Self type of this impl is &T and so the method argument
    // is actually &&T! That makes this impl lower priority during method
    // resolution if the impl that accepts &String would also apply.
    impl<T: Display> DisplayToString for &T {
        fn my_to_string(&self) -> String {
            println!("called blanket impl");

            let mut buf = String::new();
            buf.write_fmt(format_args!("{}", self)).unwrap();
            buf.shrink_to_fit();
            buf
        }
    }

    pub trait StringToString {
        fn my_to_string(&self) -> String;
    }

    // Specialized impl to bypass the relatively expensive std::fmt machinery.
    //
    // The method argument is typed &String.
    impl StringToString for String {
        fn my_to_string(&self) -> String {
            println!("called specialized impl");

            self.clone()
        }
    }

    macro_rules! convert_to_strings {
        ($($e:expr),*) => {
            [$(
                (&$e).my_to_string()
            ),*]
        };
    }

    pub(super) fn main() {
        let owned_string = "hacks".to_owned();
        let strings = convert_to_strings![1, "&str", owned_string];
        println!("{:?}", strings);
    }
}

/*
    The call `(&"oh no!").anyhow_kind().new("oh no!")` is resolved like this:
        1. By value: <&&str>::anyhow_kind(&&str)
            Not found.
        2. Autoref: <&&str>::anyhow_kind(&<&&str>)
            Found <&&str as DisplayKind>::anyhow_kind(&self) -> DisplayTag
            1. By Value: DisplayTag::new(DisplayTag, &str)
                Found.
                Executes:
                    let error: &str = "oh no!";
                    let receiver: &&str = &error;
                    let kind: DisplayTag = <&&str as DisplayKind>::anyhow_kind(&receiver);
                    DisplayTag::new(kind, error)

    The call `(&io_error).anyhow_kind().new(io_error)` is resolved like this:
        1. By value: <&std::io::Error>::anyhow_kind(&std::io::Error)
            Not found.
        2. Autoref: <&std::io::Error>::anyhow_kind(&<&std::io::Error>)
            Not found.
        3. Deref: <&std::io::Error as Deref>::Target = std::io::Error
            1. By value: <std::io::Error>::anyhow_kind(std::io::Error)
                Not found.
            2. Autoref: <std::io::Error>::anyhow_kind(&std::io::Error)
                Found <std::io::Error as StdErrorKind>::anyhow_kind(&self) -> StdErrorTag
                1. By value: StdErrorTag::new(StdErrorTag, std::io::Error)
                    Found StdErrorTag::new<E: StdError>(self, E) -> Error
                    Executes:
                        let error: std::io::Error = io_error;
                        let receiver: &std::io::Error = &error;
                        let e_ref: &std::io::Error = <<&std::io::Error as Deref>::deref(&receiver);
                        let kind: StdErrorTag = <std::io::Error as StdErrorKind>::anyhow_kind(&*e_ref);
                        StdErrorTag::new(kind, error)
*/

mod by_value {
    use std::error::Error as StdError;
    use std::fmt::Display;

    pub struct Error(/* ... */);

    // Our two constructors. The first is more general.
    impl Error {
        pub(crate) fn from_fmt<T: Display>(error: T) -> Self {
            println!("called Error::from_fmt");
            Error {}
        }
        pub(crate) fn from_std_error<T: StdError>(error: T) -> Self {
            _ = error.source(); // it works!
            println!("called Error::from_std_error");
            Error {}
        }
    }

    macro_rules! anyhow {
        ($err:expr) => {{
            #[allow(unused_imports)]
            use $crate::autoref_specialization::original::by_value::{DisplayKind, StdErrorKind};
            match $err {
                error => (&error).anyhow_kind().new(error),
            }
        }};
    }

    // If the arg implements Display but not StdError, anyhow_kind() will
    // return this tag.
    struct DisplayTag;

    trait DisplayKind {
        #[inline]
        fn anyhow_kind(&self) -> DisplayTag {
            DisplayTag
        }
    }

    // Requires one extra autoref to call! Lower priority than StdErrorKind.
    impl<T: Display> DisplayKind for &T {}

    impl DisplayTag {
        #[inline]
        fn new<M: Display>(self, message: M) -> Error {
            Error::from_fmt(message)
        }
    }

    // If the arg implements StdError (and thus also Display), anyhow_kind()
    // will return this tag.
    struct StdErrorTag;

    trait StdErrorKind {
        #[inline]
        fn anyhow_kind(&self) -> StdErrorTag {
            StdErrorTag
        }
    }

    // Does not require any autoref if called as (&error).anyhow_kind().
    impl<T: StdError> StdErrorKind for T {}

    impl StdErrorTag {
        #[inline]
        fn new<E: StdError>(self, error: E) -> Error {
            Error::from_std_error(error)
        }
    }

    pub(super) fn main() {
        // Turn a &str into an error.
        // &str implements Display but not std::error::Error.
        let _err = anyhow!("oh no!");

        // Turn an existing std::error::Error value into our error without
        // losing its source() and backtrace() if there is one.
        let io_error = std::fs::read("/tmp/nonexist").unwrap_err();
        let _err = anyhow!(io_error);
    }
}

pub(super) fn demo() {
    by_reference::main();
    by_value::main();
}
