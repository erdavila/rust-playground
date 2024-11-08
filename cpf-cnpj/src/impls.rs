macro_rules! from_iter {
    ($parser:ident) => {
        fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, Error> {
            let mut iter = iter.into_iter().enumerate();

            let output = $parser::parse(&mut iter)?;
            $parser::ensure_all_consumed(&mut iter)?;

            Ok(output)
        }
    };
}

macro_rules! chars {
    () => {
        #[must_use]
        pub fn char(self, index: usize) -> char {
            self.0[index].into()
        }

        pub fn chars(self) -> [char; Self::LENGTH] {
            self.0.map(Into::into)
        }
    };
}

macro_rules! unchecked_id {
    ($name:ident, $length:literal, $checked:ident, $parser:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name(pub(crate) [u8; Self::LENGTH]);

        impl $name {
            pub const LENGTH: usize = $length;

            from_iter!($parser);

            #[must_use]
            pub fn calculate_check_digits(self) -> $crate::CheckDigits {
                $crate::CheckDigits::from(self)
            }

            #[must_use]
            pub fn checked(self) -> $checked {
                self.with_check_digits()
            }

            #[must_use]
            pub fn with_check_digits(self) -> $checked {
                let check_digits = self.calculate_check_digits();
                let bytes = array::from_fn(|i| {
                    if i < Self::LENGTH {
                        self.0[i]
                    } else {
                        check_digits.0[i - Self::LENGTH]
                    }
                });

                $checked(bytes)
            }

            chars!();
        }

        from_str_and_try_from!($name);
    };
}

macro_rules! checked_id {
    ($name:ident, $unchecked:ident, $unchecked_parser:ident) => {
        #[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
        pub struct $name(pub(crate) [u8; Self::LENGTH]);

        impl $name {
            pub const LENGTH: usize = $unchecked::LENGTH + $crate::CheckDigits::LENGTH;

            fn from_iter(iter: impl IntoIterator<Item = char>) -> Result<Self, $crate::Error> {
                use $crate::{check_digits::CheckDigitsParser, parser::Parser, Error};

                let mut iter = iter.into_iter().enumerate();
                let unchecked_cpf = $unchecked_parser::parse(&mut iter)?;
                let check_digits = CheckDigitsParser::parse(&mut iter)?;
                CheckDigitsParser::ensure_all_consumed(&mut iter)?;

                if unchecked_cpf.calculate_check_digits() != check_digits {
                    return Err(Error::WrongCheckDigits);
                }

                let mut bytes = [b'\0'; Self::LENGTH];
                bytes[..$unchecked::LENGTH].copy_from_slice(&unchecked_cpf.0);
                bytes[$unchecked::LENGTH..].copy_from_slice(&check_digits.0);

                Ok($name(bytes))
            }

            #[must_use]
            pub fn unchecked(self) -> $unchecked {
                self.without_check_digits()
            }

            #[must_use]
            pub fn without_check_digits(self) -> $unchecked {
                let mut bytes = [b'\0'; $unchecked::LENGTH];
                bytes.copy_from_slice(&self.0[..$unchecked::LENGTH]);
                $unchecked(bytes)
            }

            #[must_use]
            pub fn check_digits(self) -> $crate::CheckDigits {
                let mut bytes = [b'\0'; $crate::CheckDigits::LENGTH];
                bytes.copy_from_slice(&self.0[$unchecked::LENGTH..]);
                $crate::CheckDigits(bytes)
            }

            chars!();
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}-{}", self.without_check_digits(), self.check_digits())
            }
        }
    };
}

macro_rules! from_str_and_try_from {
    ($name:ident) => {
        impl FromStr for $name {
            type Err = $crate::Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Self::from_iter(s.chars())
            }
        }

        impl TryFrom<&str> for $name {
            type Error = $crate::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::from_iter(value.chars())
            }
        }

        impl TryFrom<String> for $name {
            type Error = $crate::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Self::from_iter(value.chars())
            }
        }

        impl TryFrom<[char; Self::LENGTH]> for $name {
            type Error = $crate::Error;

            fn try_from(value: [char; Self::LENGTH]) -> Result<Self, Self::Error> {
                Self::from_iter(value)
            }
        }
    };
}
