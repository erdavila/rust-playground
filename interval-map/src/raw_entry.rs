use core::fmt::Debug;

use crate::interval::{Interval, LeftEndpoint, RightEndpoint};

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct RawEntry<K, V> {
    pub interval: Interval<K>,
    pub value: V,
}

impl<K, V> RawEntry<K, V> {
    pub fn new(interval: Interval<K>, value: V) -> Self {
        RawEntry { interval, value }
    }

    pub fn left(&self) -> &LeftEndpoint<K> {
        &self.interval.left
    }

    pub fn left_mut(&mut self) -> &mut LeftEndpoint<K> {
        &mut self.interval.left
    }

    pub fn right(&self) -> &RightEndpoint<K> {
        &self.interval.right
    }

    pub fn right_mut(&mut self) -> &mut RightEndpoint<K> {
        &mut self.interval.right
    }
}

impl<K, V> From<(Interval<K>, V)> for RawEntry<K, V> {
    fn from((interval, value): (Interval<K>, V)) -> Self {
        RawEntry { interval, value }
    }
}

impl<K, V> From<RawEntry<K, V>> for (Interval<K>, V) {
    fn from(interval_and_value: RawEntry<K, V>) -> Self {
        let RawEntry { interval, value } = interval_and_value;
        (interval, value)
    }
}

impl<K: Debug, V: Debug> Debug for RawEntry<K, V> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.interval.fmt(f)?;
        f.write_str(": ")?;
        self.value.fmt(f)?;
        Ok(())
    }
}
