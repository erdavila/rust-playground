use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) struct TotalOrdF32(f32);

impl TotalOrdF32 {
    pub(crate) fn from_ref<T: Copy + Into<Self>>(x: &T) -> Self {
        (*x).into()
    }
}

impl Ord for TotalOrdF32 {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for TotalOrdF32 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for TotalOrdF32 {}

impl<T> From<(T, f32)> for TotalOrdF32 {
    fn from((_, value): (T, f32)) -> Self {
        TotalOrdF32(value)
    }
}

impl<T, U> From<(T, U, f32)> for TotalOrdF32 {
    fn from((_, _, value): (T, U, f32)) -> Self {
        TotalOrdF32(value)
    }
}
