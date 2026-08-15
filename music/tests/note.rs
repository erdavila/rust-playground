use std::array;

use music::accidental::{Accidental, AccidentalIndexed};
use music::modulo::ModuloNumbered as _;
use music::name::{NAME_COUNT, Name};
use music::note::{NOTE_COUNT, Note, NoteNumber};
use music::scale::Scale;

#[test]
fn names() {
    let c_name_number = AccidentalIndexed {
        flat: NoteNumber::from(NOTE_COUNT - 1),
        natural: NoteNumber::from(0usize),
        sharp: NoteNumber::from(1usize),
    };

    let names: [_; NOTE_COUNT] = array::from_fn(|i| Note::from(i).names());

    for (name_idx, offset) in Scale::Major.note_offsets().into_iter().enumerate() {
        for accid in Accidental::values() {
            let note_idx = c_name_number[accid] + offset;
            let name = names[note_idx.usize_value()][accid];
            let expected_name = Name::C.succ_by(name_idx);

            assert_eq!(
                name,
                Some(expected_name),
                "name_idx={name_idx}, offset={offset}, accid={accid:?}",
            );
        }
    }

    let mut counts: AccidentalIndexed<usize> = AccidentalIndexed::default();
    for name in names {
        for (accid, name) in name {
            if name.is_some() {
                counts[accid] += 1;
            }
        }
    }
    assert_eq!(counts.flat, NAME_COUNT);
    assert_eq!(counts.natural, NAME_COUNT);
    assert_eq!(counts.sharp, NAME_COUNT);
}
