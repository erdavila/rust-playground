use music::note::{NOTE_COUNT, Note};
use music::scale::Scale;

fn main() {
    for n in 0..NOTE_COUNT {
        let first_note = Note::from(n);
        for (scale_accid, named_notes) in Scale::Major.named_notes(first_note) {
            if let Some(named_notes) = named_notes {
                print!("{}) {scale_accid}: ", n + 1);
                for (i, named_note) in named_notes.into_iter().enumerate() {
                    if i > 0 {
                        print!(", ");
                    }
                    print!(" {named_note}");
                }
                println!();
            }
        }

        println!();
    }
}
