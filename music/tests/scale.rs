use music::accidental::Accidental;
use music::interval::PERFECT_FIFTH;
use music::modulo::ModuloNumbered as _;
use music::note::{NOTE_COUNT, Note};
use music::scale::{NOTES_PER_SCALE, Scale};

#[test]
fn scale_note_names() {
    for number in 0..NOTE_COUNT {
        let scale_first_note = Note::from(number);
        let scale_note_names = Scale::Major.named_notes(scale_first_note);

        macro_rules! assert_accidentals {
            ($accidental:expr) => {{
                let Some(names) = &scale_note_names[$accidental] else {
                    panic!("{:?}", $accidental);
                };

                for (offset, &named_note) in Scale::Major.note_offsets().into_iter().zip(names) {
                    assert_eq!(Note::from(named_note), scale_first_note.succ_by(offset), "[expected note] named_note={named_note:?}, scale_first_note={scale_first_note:?}, offset={offset}");
                    assert!(named_note.accidental == $accidental || named_note.accidental == Accidental::Natural, "[expected accidental] named_note={named_note:?}, offset={offset}");
                }

                names.iter().filter(|named_note| named_note.accidental == $accidental).count()
            }};
        }

        let circle_of_fifths_index =
            (scale_first_note.modulo_number() * PERFECT_FIFTH.semitones()).usize_value();

        if circle_of_fifths_index == 0 {
            // C
            let count = assert_accidentals!(Accidental::Natural);
            assert_eq!(count, NOTES_PER_SCALE);

            assert_eq!(scale_note_names.flat, None);
            assert_eq!(scale_note_names.sharp, None);
        } else {
            macro_rules! assert_non_natural_accidentals {
                ($accidental:expr, $expected:expr) => {
                    if $expected <= NOTES_PER_SCALE {
                        let count = assert_accidentals!($accidental);
                        assert_eq!(count, $expected, "[expected count]");
                    } else {
                        assert_eq!(scale_note_names[$accidental], None, "[no expected notes]");
                    }
                };
            }

            let expected_sharps = circle_of_fifths_index;
            let expected_flats = NOTE_COUNT - expected_sharps;

            assert_non_natural_accidentals!(Accidental::Sharp, expected_sharps);
            assert_non_natural_accidentals!(Accidental::Flat, expected_flats);
            assert_eq!(scale_note_names[Accidental::Natural], None);
        }
    }
}
