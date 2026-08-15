use core::ops::{Add, Mul, Sub};

// Having `usize` 16-bit or wider ensures operations can be done without overflow.
const _: () = assert!(usize::BITS >= 16);

pub trait ModuloValue: Into<usize> {}
impl ModuloValue for u8 {}
impl ModuloValue for usize {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Modulo<const N: u8>(u8);

impl<const N: u8> Modulo<N> {
    /// # Panics
    #[must_use]
    pub fn new(value: impl ModuloValue) -> Self {
        assert!(N > 0);
        let value = value.into() % usize::from(N);

        #[expect(clippy::cast_possible_truncation)]
        Self(value as u8)
    }

    #[must_use]
    pub fn u8_value(self) -> u8 {
        self.0
    }

    #[must_use]
    pub fn usize_value(self) -> usize {
        usize::from(self.0)
    }
}

impl<const N: u8, V: ModuloValue> Add<V> for Modulo<N> {
    type Output = Self;

    fn add(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let value = self.usize_value();
        Self::from(value + rhs)
    }
}

impl<const N: u8, V: ModuloValue> Sub<V> for Modulo<N> {
    type Output = Self;

    fn sub(self, rhs: V) -> Self::Output {
        let rhs = rhs.into() % usize::from(N);
        let value = self.usize_value();
        Self::from(usize::from(N) + value - rhs)
    }
}

impl<const N: u8, V: ModuloValue> Mul<V> for Modulo<N> {
    type Output = Self;

    fn mul(self, rhs: V) -> Self::Output {
        let rhs = rhs.into();
        let value = self.usize_value();
        Self::from(value * rhs)
    }
}

impl<const N: u8, V: ModuloValue> From<V> for Modulo<N> {
    fn from(value: V) -> Self {
        Self::new(value)
    }
}

pub trait ModuloNumbered<const N: u8>: Copy + From<Modulo<N>> + From<u8> + From<usize> {
    #[must_use]
    fn new(value: impl ModuloValue) -> Self {
        let number = Modulo::new(value);
        Self::from(number)
    }

    #[must_use]
    fn modulo_number(self) -> Modulo<N>;

    #[must_use]
    fn u8_value(self) -> u8 {
        self.modulo_number().u8_value()
    }

    #[must_use]
    fn usize_value(self) -> usize {
        self.modulo_number().usize_value()
    }

    #[must_use]
    fn succ(self) -> Self {
        self.succ_by(1usize)
    }

    #[must_use]
    fn pred(self) -> Self {
        self.pred_by(1usize)
    }

    #[must_use]
    fn succ_by(self, amount: impl ModuloValue) -> Self {
        Self::from(self.modulo_number() + amount)
    }

    #[must_use]
    fn pred_by(self, amount: impl ModuloValue) -> Self {
        Self::from(self.modulo_number() - amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        const N: u8 = 4;

        for x in 0..3 * N {
            let x_mod = Modulo::<N>::new(x);

            for y in 0..3 * N {
                let output = x_mod + y;

                let expected = (x + y) % N;
                assert_eq!(output.u8_value(), expected, "x={x}, y={y}");
            }
        }
    }

    #[test]
    fn sub() {
        const N: u8 = 4;

        for mut x in 0..3 * N {
            let x_mod = Modulo::<N>::new(x);

            for y in 0..3 * N {
                let output = x_mod - y;

                let expected = {
                    while x < y {
                        x += N;
                    }
                    (x - y) % N
                };
                assert_eq!(output.u8_value(), expected, "x={x}, y={y}");
            }
        }
    }
}
