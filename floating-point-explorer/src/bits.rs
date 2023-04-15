use std::fmt::Display;

pub struct Bits {
    pub value: u128,
    length: usize,
}

impl Bits {
    pub fn extract_from(source: &mut u128, length: usize) -> Self {
        let mut mask = 0x01_u128; // Sequence of several zeroes followed by 1 one
        mask = !mask; // Sequence of several ones followed by 1 zero
        mask <<= length - 1; // Sequence of several ones followed by `length` zeroes
        mask = !mask; // Sequence of several zeroes followed by `length` ones

        let value = *source & mask;
        *source >>= length;

        Bits { value, length }
    }
}

impl Display for Bits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:0len$b}", self.value, len = self.length)
    }
}
