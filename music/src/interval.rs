#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Interval {
    semitones: u8,
}

impl Interval {
    #[must_use]
    pub fn semitones(self) -> u8 {
        self.semitones
    }

    #[must_use]
    pub fn invert(self) -> Interval {
        Interval {
            semitones: 12 - self.semitones,
        }
    }
}

pub const PERFECT_FIFTH: Interval = Interval { semitones: 7 };
