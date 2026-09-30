use rand::{Rng, RngExt};

use crate::unit::{GENES, Unit};

const MUTATION_PROBABILITY: f64 = 0.2;
/// Maximum change applied to a gene when it mutates.
const MUTATION_STEP: f64 = 0.1;
const TOURNAMENT_SIZE: usize = 3;

/// A non-empty group of units that evolves generation by generation.
pub struct Population {
    units: Vec<Unit>,
}

impl Population {
    pub fn random(size: usize, rng: &mut impl Rng) -> Self {
        assert!(size > 0, "population must not be empty");
        Self {
            units: (0..size).map(|_| Unit::random(rng)).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.units.len()
    }

    pub fn best(&self) -> &Unit {
        self.units
            .iter()
            .min_by(|a, b| a.fitness().total_cmp(&b.fitness()))
            .expect("population is non-empty")
    }

    /// Builds the next generation: the best unit survives unchanged (elitism),
    /// the rest are bred from tournament-selected parents.
    pub fn evolve(&self, rng: &mut impl Rng) -> Self {
        let mut units = Vec::with_capacity(self.len());
        units.push(*self.best());

        while units.len() < self.len() {
            let parent1 = self.tournament(rng);
            let parent2 = self.tournament(rng);
            // Start at 1 so the child always inherits from both parents.
            let child = parent1.crossover(parent2, rng.random_range(1..GENES));

            let mut mutated = child;
            mutated.mutate(rng, MUTATION_PROBABILITY, MUTATION_STEP);

            units.push(if mutated.fitness() < child.fitness() { mutated } else { child });
        }

        Self { units }
    }

    fn tournament(&self, rng: &mut impl Rng) -> &Unit {
        (0..TOURNAMENT_SIZE)
            .map(|_| &self.units[rng.random_range(..self.len())])
            .min_by(|a, b| a.fitness().total_cmp(&b.fitness()))
            .expect("TOURNAMENT_SIZE is non-zero")
    }
}
