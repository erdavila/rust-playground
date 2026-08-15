use core::fmt::Display;
use core::ops::Neg;

use enum_indexed::indexed_struct::IndexedStruct;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default, IndexedStruct)]
pub enum Accidental {
    Flat,
    #[default]
    Natural,
    Sharp,
}

impl Accidental {
    #[must_use]
    pub fn values() -> [Accidental; 3] {
        [Accidental::Flat, Accidental::Natural, Accidental::Sharp]
    }
}

impl Neg for Accidental {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Accidental::Flat => Accidental::Sharp,
            Accidental::Natural => Accidental::Natural,
            Accidental::Sharp => Accidental::Flat,
        }
    }
}

impl Display for Accidental {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let (s, s_alt) = match self {
            Accidental::Flat => ("b", "♭"),
            Accidental::Natural => ("", "♮"),
            Accidental::Sharp => ("#", "♯"),
        };

        f.write_str(if f.alternate() { s_alt } else { s })
    }
}
