use enum_indexed::indexed_struct::EnumIndexed as _;

use crate::accidental::{Accidental, AccidentalIndexed};
use crate::modulo::ModuloNumbered as _;
use crate::name::Name;
use crate::note::{NamedNote, Note};

pub const NOTES_PER_SCALE: usize = 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Scale {
    Major,
}

impl Scale {
    #[must_use]
    pub fn note_offsets(self) -> [u8; NOTES_PER_SCALE] {
        match self {
            Scale::Major => [0, 2, 4, 5, 7, 9, 11],
        }
    }

    #[must_use]
    pub fn notes(self, first_note: Note) -> [Note; NOTES_PER_SCALE] {
        self.note_offsets().map(|offset| first_note.succ_by(offset))
    }

    #[must_use]
    pub fn named_notes(
        self,
        first_note: Note,
    ) -> AccidentalIndexed<Option<[NamedNote; NOTES_PER_SCALE]>> {
        let notes = self.notes(first_note);
        let first_note_names = notes[0].names();

        AccidentalIndexed::from_fn(|accid| {
            first_note_names.into_values().find_map(|first_note_name| {
                let first_note_name = first_note_name?;
                try_note_names(accid, notes, first_note_name)
            })
        })
    }
}

fn try_note_names(
    accid: Accidental,
    notes: [Note; NOTES_PER_SCALE],
    first_note_name: Name,
) -> Option<[NamedNote; NOTES_PER_SCALE]> {
    let note_accids: &[_] = if accid == Accidental::Natural {
        &[accid]
    } else {
        &[accid, Accidental::Natural]
    };

    // The initial values don't matter.
    let mut names = [NamedNote {
        name: Name::C,
        accidental: Accidental::Natural,
    }; NOTES_PER_SCALE];

    let mut any_non_natural = false;

    for (i, note) in notes.into_iter().enumerate() {
        let name = first_note_name.succ_by(i);
        let &accidental = note_accids
            .iter()
            .find(|&&accid| Note::from((name, accid)) == note)?;

        names[i] = NamedNote { name, accidental };
        any_non_natural |= accidental != Accidental::Natural;
    }

    (accid == Accidental::Natural || any_non_natural).then_some(names)
}
