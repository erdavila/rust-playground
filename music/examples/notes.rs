use music::note::{NOTE_COUNT, Note};

fn main() {
    for n in 0..NOTE_COUNT {
        let note = Note::from(n);
        println!("{}) {note}", n + 1);
    }
}
