pub struct RawDeterministicChooser {
    stats: Vec<ItemStats>,
    weights_vector_length: f64,
}

impl RawDeterministicChooser {
    #[expect(clippy::missing_panics_doc)]
    pub fn new(weights: impl IntoIterator<Item = f64>) -> Self {
        let stats: Vec<_> = weights
            .into_iter()
            .map(|weight| {
                assert!(weight >= 0.0, "all weights must be non-negative");
                ItemStats { weight, count: 0 }
            })
            .collect();

        let weights_vector_length = Self::vector_length(stats.iter().map(|s| s.weight));
        assert!(
            weights_vector_length > 0.0,
            "at least one weight must be positive"
        );

        RawDeterministicChooser {
            stats,
            weights_vector_length,
        }
    }

    #[must_use]
    pub fn stats(&self) -> &Vec<ItemStats> {
        &self.stats
    }

    fn cosine_similarity(&self, counts: Vec<f64>) -> f64 {
        let weights = self.stats.iter().map(|s| s.weight);
        let dot_product: f64 = weights.zip(counts.iter()).map(|(x, y)| x * y).sum();
        let counts_vec_len = Self::vector_length(counts);
        dot_product / (self.weights_vector_length * counts_vec_len)
    }

    fn vector_length(v: impl IntoIterator<Item = f64>) -> f64 {
        v.into_iter().map(|x| x * x).sum::<f64>().sqrt()
    }
}

impl Iterator for RawDeterministicChooser {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let (index, _) = (0..self.stats.len())
            .map(|i| {
                let counts = self.stats.iter().enumerate().map(|(j, s)| {
                    let mut count = s.count;
                    if i == j {
                        count += 1;
                    }
                    #[expect(clippy::cast_precision_loss)]
                    let count = count as f64;
                    count
                });

                let similarity = self.cosine_similarity(counts.collect());

                (i, similarity)
            })
            .max_by(|(_i1, similarity1), (_i2, similarity2)| similarity1.total_cmp(similarity2))
            .unwrap();

        self.stats[index].count += 1;
        Some(index)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ItemStats {
    pub weight: f64,
    pub count: usize,
}
