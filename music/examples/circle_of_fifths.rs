use music::accidental::Accidental;
use music::interval::PERFECT_FIFTH;
use music::modulo::ModuloNumbered as _;
use music::name::{NAME_COUNT, Name};
use music::note::{NOTE_COUNT, NamedNote, Note};
use music::scale::Scale;

fn main() {
    let mut major_scale_first_note = Note::from(0usize);

    for n in 0..NOTE_COUNT {
        let named_notes = Scale::Major.named_notes(major_scale_first_note);

        let accids = [
            (Accidental::Natural, None::<fn(Name) -> isize>),
            (Accidental::Sharp, Some(sharp_inclusion_order)),
            (Accidental::Flat, Some(flat_inclusion_order)),
        ];

        for (accid, order) in accids {
            if let Some(named_notes) = named_notes[accid] {
                let major_scale = named_notes[0];
                let minor_scale = major_scale_first_note
                    .succ_by(9usize)
                    .names()
                    .into_iter()
                    .find_map(|(accidental, name)| {
                        let name = name?;
                        (accidental == accid
                            || accid == Accidental::Natural
                            || accidental == Accidental::Natural)
                            .then_some(NamedNote { name, accidental })
                    })
                    .unwrap_or_else(|| unreachable!());

                print!("{}) [{major_scale}/{minor_scale}m] ", n + 1);

                if let Some(order) = order {
                    let mut accid_note_names: Vec<_> = named_notes
                        .into_iter()
                        .filter_map(|named_note| {
                            (named_note.accidental == accid).then_some(named_note.name)
                        })
                        .collect();
                    accid_note_names.sort_by_key(|&name| order(name));

                    print!("{}{}: ", accid_note_names.len(), accid);
                    for (i, name) in accid_note_names.into_iter().enumerate() {
                        if i > 0 {
                            print!(", ");
                        }
                        print!("{name}");
                    }
                } else {
                    print!("♮");
                }

                println!();
            }
        }

        println!();
        major_scale_first_note += PERFECT_FIFTH;
    }
}

fn sharp_inclusion_order(name: Name) -> isize {
    match name {
        Name::F => 0,
        Name::C => 1,
        Name::G => 2,
        Name::D => 3,
        Name::A => 4,
        Name::E => 5,
        Name::B => 6,
    }
}

fn flat_inclusion_order(name: Name) -> isize {
    NAME_COUNT.cast_signed() - sharp_inclusion_order(name)
}
