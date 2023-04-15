pub use raw::ItemStats;
use raw::RawDeterministicChooser;

pub mod deterministic_bool_chooser;
pub mod raw;

pub struct DeterministicChooser<T: Clone> {
    raw: RawDeterministicChooser,
    values: Vec<T>,
}

impl<T: Clone> DeterministicChooser<T> {
    pub fn new(values_and_weights: impl IntoIterator<Item = (T, f64)>) -> Self {
        let (mut values, weights): (Vec<_>, Vec<_>) = values_and_weights.into_iter().unzip();
        values.shrink_to_fit();
        let raw = RawDeterministicChooser::new(weights);
        DeterministicChooser { raw, values }
    }

    pub fn stats(&self) -> Vec<(T, ItemStats)> {
        let values = self.values.iter().cloned();
        let stats = self.raw.stats().iter().cloned();
        values.zip(stats).collect()
    }
}

impl<T: Clone> Iterator for DeterministicChooser<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|index| self.values[index].clone())
    }
}
