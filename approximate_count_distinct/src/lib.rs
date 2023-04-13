//! Implementation of Distinct Elements in Streams: An Algorithm for the (Text) Book
//!
//! URL: https://arxiv.org/pdf/2301.10191.pdf

use rand::rngs::ThreadRng;
use rand::{self, Rng};
use std::collections::HashSet;
use std::hash::Hash;

pub struct ApproximateCountDistinct<T> {
    seen: HashSet<T>,
    params: Params,
    probability: Probability,
}

impl<T> ApproximateCountDistinct<T>
where
    T: Eq + Hash,
{
    pub fn with_max_set_size(max_set_size: usize) -> Self {
        let params = Params::with_max_set_size(max_set_size);
        Self::with_params(params)
    }

    pub fn with_params(params: Params) -> Self {
        Self {
            seen: HashSet::new(),
            params,
            probability: Probability {
                rng: rand::thread_rng(),
            },
        }
    }

    pub fn see_many(&mut self, values: impl IntoIterator<Item = T>) {
        for value in values {
            self.see(value);
        }
    }

    pub fn see(&mut self, value: T) {
        if self.probability.of(self.params.factor) {
            self.seen.insert(value);

            if self.seen.len() >= self.params.max_set_size {
                self.seen
                    .retain(|_| self.probability.of(self.params.factor_adjustment));
                self.params.factor *= self.params.factor_adjustment;
            }
        } else {
            self.seen.remove(&value);
        }
    }

    pub fn approximate_count_distinct(&self) -> f64 {
        self.seen.len() as f64 / self.params.factor
    }

    pub fn params(&self) -> Params {
        self.params
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Params {
    max_set_size: usize,
    factor: f64,
    factor_adjustment: f64,
}

impl Params {
    pub fn with_max_set_size(max_set_size: usize) -> Self {
        Params {
            factor: 1.0,
            factor_adjustment: 0.5,
            max_set_size,
        }
    }

    pub fn with_unlimited_set_size() -> Self {
        Self::with_max_set_size(usize::MAX)
    }

    pub fn set_max_set_size(mut self, max_set_size: usize) -> Self {
        assert!(max_set_size > 0, "max_set_size must be greater than zero");
        self.max_set_size = max_set_size;
        self
    }

    pub fn max_set_size(&self) -> usize {
        self.max_set_size
    }

    pub fn set_factor(mut self, factor: f64) -> Self {
        assert!(factor > 0.0, "factor must be greater than zero");
        assert!(factor <= 1.0, "factor must be less than or equal to one");
        self.factor = factor;
        self
    }

    pub fn factor(&self) -> f64 {
        self.factor
    }

    pub fn set_factor_adjustment(mut self, factor_adjustment: f64) -> Self {
        assert!(
            factor_adjustment > 0.0,
            "factor_adjustment must be greater than zero"
        );
        assert!(
            factor_adjustment < 1.0,
            "factor_adjustment must be less than one"
        );
        self.factor_adjustment = factor_adjustment;
        self
    }

    pub fn factor_adjustment(&self) -> f64 {
        self.factor_adjustment
    }
}

struct Probability {
    rng: ThreadRng,
}

impl Probability {
    fn of(&mut self, p: f64) -> bool {
        let x: f64 = self.rng.gen();
        x < p
    }
}
