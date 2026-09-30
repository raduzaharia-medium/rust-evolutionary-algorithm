use super::*;
use crate::unit::{GENES, Unit};

fn unit(value: f64) -> Unit {
    Unit { genes: [value; GENES] }
}

#[test]
fn fitness_is_gene_sum() {
    assert_eq!(unit(0.5).fitness(), 0.5 * GENES as f64);
}

#[test]
fn crossover_splits_at_point() {
    let child = unit(0.0).crossover(&unit(1.0), 3);
    assert_eq!(child.genes[..3], [0.0; 3]);
    assert_eq!(child.genes[3..], [1.0; GENES - 3]);
}

#[test]
fn mutation_probability_zero_changes_nothing() {
    let mut mutated = unit(0.5);
    mutated.mutate(&mut rand::rng(), 0.0, 0.1);
    assert_eq!(mutated, unit(0.5));
}

#[test]
fn mutation_nudges_within_step_and_bounds() {
    let mut rng = rand::rng();
    for _ in 0..100 {
        let mut nudged = unit(0.5);
        nudged.mutate(&mut rng, 1.0, 0.1);
        assert!(nudged.genes.iter().all(|g| (0.4..=0.6).contains(g)));

        let mut low = unit(0.0);
        low.mutate(&mut rng, 1.0, 0.1);
        assert!(low.genes.iter().all(|g| (0.0..=0.1).contains(g)));
    }
}

#[test]
fn evolve_keeps_size_and_never_loses_best() {
    let mut rng = rand::rng();
    let mut population = Population::random(POPULATION_SIZE, &mut rng);
    let mut best = population.best().fitness();

    for _ in 0..100 {
        population = population.evolve(&mut rng);
        assert_eq!(population.len(), POPULATION_SIZE);
        let now = population.best().fitness();
        assert!(now <= best);
        best = now;
    }
}
