use crate::Size;
use crate::enum_map::EnumMapKey;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum Algo {
    Naive,
    Kahan,
    Algeb,
}
impl Algo {
    pub(crate) fn sum(self, size: Size, numbers: &[f32]) -> f32 {
        match (self, size) {
            (Algo::Naive, Size::F32) => Self::naive_f32(numbers),
            (Algo::Naive, Size::F64) => Self::naive_f64(numbers),
            (Algo::Kahan, Size::F32) => Self::kahan_f32(numbers),
            (Algo::Kahan, Size::F64) => Self::kahan_f64(numbers),
            (Algo::Algeb, Size::F32) => Self::algebraic_f32(numbers),
            (Algo::Algeb, Size::F64) => Self::algebraic_f64(numbers),
        }
    }

    fn naive_f32(numbers: &[f32]) -> f32 {
        numbers.iter().sum()
    }

    fn kahan_f32(numbers: &[f32]) -> f32 {
        let mut sum = 0.0;
        let mut c = 0.0;

        for &x in numbers {
            let y = x - c;
            let t = sum + y;
            c = (t - sum) - y;
            sum = t;
        }

        sum
    }

    fn algebraic_f32(numbers: &[f32]) -> f32 {
        numbers.iter().fold(0.0, |x, &y| x.algebraic_add(y))
    }

    fn naive_f64(numbers: &[f32]) -> f32 {
        let sum: f64 = numbers.iter().map(|n| f64::from(*n)).sum();

        #[expect(clippy::cast_possible_truncation)]
        let output = sum as f32;

        output
    }

    fn kahan_f64(numbers: &[f32]) -> f32 {
        let mut sum = 0.0;
        let mut c = 0.0;

        for &x in numbers {
            let y = f64::from(x) - c;
            let t = sum + y;
            c = (t - sum) - y;
            sum = t;
        }

        #[expect(clippy::cast_possible_truncation)]
        let output = sum as f32;

        output
    }

    fn algebraic_f64(numbers: &[f32]) -> f32 {
        let sum: f64 = numbers
            .iter()
            .map(|n| f64::from(*n))
            .fold(0.0, f64::algebraic_add);

        #[expect(clippy::cast_possible_truncation)]
        let output = sum as f32;

        output
    }
}
impl EnumMapKey<3> for Algo {
    fn all() -> [Self; 3] {
        [Algo::Naive, Algo::Kahan, Algo::Algeb]
    }
}
