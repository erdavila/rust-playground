use core::fmt::Display;
use core::ops::{Add, AddAssign, Sub};

use enum_indexed::indexed_struct::EnumIndexed as _;

use crate::accidental::{Accidental, AccidentalIndexed};
use crate::interval::Interval;
use crate::modulo::{Modulo, ModuloNumbered, ModuloValue};
use crate::name::Name;
use crate::scale::Scale;

const NOTE_COUNT_AS_U8: u8 = 12;
pub const NOTE_COUNT: usize = NOTE_COUNT_AS_U8 as usize;

pub type NoteNumber = Modulo<NOTE_COUNT_AS_U8>;

const NATURAL_NOTES: [Option<Name>; NOTE_COUNT] = [
    Some(Name::C),
    None,
    Some(Name::D),
    None,
    Some(Name::E),
    Some(Name::F),
    None,
    Some(Name::G),
    None,
    Some(Name::A),
    None,
    Some(Name::B),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Note(NoteNumber);

impl Note {
    #[must_use]
    pub fn names(self) -> AccidentalIndexed<Option<Name>> {
        AccidentalIndexed::from_fn(|accid| {
            // "Removes" the accidental
            let natural_note = self - accid;
            NATURAL_NOTES[natural_note.0.usize_value()]
        })
    }
}

impl ModuloNumbered<NOTE_COUNT_AS_U8> for Note {
    fn modulo_number(self) -> NoteNumber {
        self.0
    }
}

impl<T: ModuloValue> From<T> for Note {
    fn from(value: T) -> Self {
        Self::from(NoteNumber::from(value))
    }
}

impl From<NoteNumber> for Note {
    fn from(value: NoteNumber) -> Self {
        Note(value)
    }
}

impl From<Name> for Note {
    fn from(name: Name) -> Self {
        let offset = Scale::Major.note_offsets()[name.usize_value()];
        Note::from(offset)
    }
}

impl From<(Name, Accidental)> for Note {
    fn from((name, accid): (Name, Accidental)) -> Self {
        name.with_accidental(accid).into()
    }
}

impl From<NamedNote> for Note {
    fn from(value: NamedNote) -> Self {
        Note::from(value.name) + value.accidental
    }
}

impl Add<Accidental> for Note {
    type Output = Self;

    fn add(self, rhs: Accidental) -> Self::Output {
        match rhs {
            Accidental::Flat => self.pred(),
            Accidental::Natural => self,
            Accidental::Sharp => self.succ(),
        }
    }
}

impl Add<Interval> for Note {
    type Output = Self;

    fn add(self, rhs: Interval) -> Self::Output {
        self.succ_by(rhs.semitones())
    }
}

impl AddAssign<Interval> for Note {
    fn add_assign(&mut self, rhs: Interval) {
        *self = *self + rhs;
    }
}

impl Sub<Accidental> for Note {
    type Output = Self;

    fn sub(self, rhs: Accidental) -> Self::Output {
        self + -rhs
    }
}

impl Display for Note {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let names = self.names();
        let mut first = true;

        let accidentals = if f.alternate() {
            [Accidental::Flat, Accidental::Natural, Accidental::Sharp]
        } else {
            [Accidental::Natural, Accidental::Sharp, Accidental::Flat]
        };

        for accid in accidentals {
            if let Some(name) = names[accid] {
                if !first {
                    write!(f, "/")?;
                }
                first = false;

                name.fmt(f)?;
                accid.fmt(f)?;
            }
        }

        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct NamedNote {
    pub name: Name,
    pub accidental: Accidental,
}

impl Display for NamedNote {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.name.fmt(f)?;
        if self.accidental != Accidental::Natural {
            self.accidental.fmt(f)?;
        }
        Ok(())
    }
}
