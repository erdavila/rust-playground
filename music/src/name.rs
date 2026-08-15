use core::fmt::{Display, Write};

use crate::accidental::Accidental;
use crate::modulo::{Modulo, ModuloNumbered, ModuloValue};
use crate::note::NamedNote;

const NAME_COUNT_AS_U8: u8 = 7;
pub const NAME_COUNT: usize = NAME_COUNT_AS_U8 as usize;

pub type NameNumber = Modulo<NAME_COUNT_AS_U8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Name {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

impl Name {
    #[must_use]
    pub const fn flat(self) -> NamedNote {
        self.with_accidental(Accidental::Flat)
    }

    #[must_use]
    pub const fn natural(self) -> NamedNote {
        self.with_accidental(Accidental::Natural)
    }

    #[must_use]
    pub const fn sharp(self) -> NamedNote {
        self.with_accidental(Accidental::Sharp)
    }

    #[must_use]
    pub const fn with_accidental(self, accidental: Accidental) -> NamedNote {
        NamedNote {
            name: self,
            accidental,
        }
    }
}

impl ModuloNumbered<NAME_COUNT_AS_U8> for Name {
    fn modulo_number(self) -> NameNumber {
        match self {
            Name::C => NameNumber::new(0usize),
            Name::D => NameNumber::new(1usize),
            Name::E => NameNumber::new(2usize),
            Name::F => NameNumber::new(3usize),
            Name::G => NameNumber::new(4usize),
            Name::A => NameNumber::new(5usize),
            Name::B => NameNumber::new(6usize),
        }
    }
}

impl<T: ModuloValue> From<T> for Name {
    fn from(value: T) -> Self {
        Self::from(NameNumber::from(value))
    }
}

impl From<NameNumber> for Name {
    fn from(number: NameNumber) -> Self {
        match number.u8_value() {
            0 => Name::C,
            1 => Name::D,
            2 => Name::E,
            3 => Name::F,
            4 => Name::G,
            5 => Name::A,
            6 => Name::B,
            _ => unreachable!(),
        }
    }
}

impl Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let c = match self {
            Name::C => 'C',
            Name::D => 'D',
            Name::E => 'E',
            Name::F => 'F',
            Name::G => 'G',
            Name::A => 'A',
            Name::B => 'B',
        };
        f.write_char(c)
    }
}
