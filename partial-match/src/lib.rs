#![no_std]

#[macro_export]
macro_rules! partial_match {
    (($e:expr) {
        $(
            $pat:pat $(if $guard:expr)? => $arm:expr $(,)?
        )*
    }) => {
        match $e {
            $(
                $pat $(if $guard)? => Some($arm),
            )*
            _ => None,
        }
    };
}
