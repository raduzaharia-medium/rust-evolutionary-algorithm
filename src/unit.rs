use rand::{Rng, RngExt};

pub const GENES: usize = 10;

/// A candidate solution. Fitness is the sum of the genes; lower is better.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Unit {
    pub genes: [f64; GENES],
}

impl Unit {
    pub fn random(rng: &mut impl Rng) -> Self {
        Self {
            genes: std::array::from_fn(|_| rng.random()),
        }
    }

    pub fn fitness(&self) -> f64 {
        self.genes.iter().sum()
    }

    /// Genes before `point` come from `self`, the rest from `other`.
    pub fn crossover(&self, other: &Self, point: usize) -> Self {
        Self {
            genes: std::array::from_fn(|i| {
                if i < point { self.genes[i] } else { other.genes[i] }
            }),
        }
    }

    /// Nudges each gene by up to `step` in either direction with the given
    /// probability, keeping it within [0, 1].
    pub fn mutate(&mut self, rng: &mut impl Rng, probability: f64, step: f64) {
        for gene in &mut self.genes {
            if rng.random_bool(probability) {
                *gene = (*gene + rng.random_range(-step..=step)).clamp(0.0, 1.0);
            }
        }
    }
}
