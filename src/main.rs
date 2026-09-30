mod population;
mod unit;

use population::Population;

const POPULATION_SIZE: usize = 20;
const MAX_GENERATIONS: usize = 20_000;
/// Stop after this many generations without the best fitness improving.
const STAGNATION_LIMIT: usize = 200;
/// Genes are drawn from [0, 1), so exactly 0.0 is practically unreachable;
/// this is the threshold we treat as perfect.
const PERFECT_FITNESS: f64 = 1e-6;

fn main() {
    let mut rng = rand::rng();
    let mut population = Population::random(POPULATION_SIZE, &mut rng);
    print_best(0, &population);

    let mut best_fitness = population.best().fitness();
    let mut stagnant_generations = 0;

    for generation in 1..=MAX_GENERATIONS {
        population = population.evolve(&mut rng);
        print_best(generation, &population);

        // Elitism means the best fitness never gets worse, only better or equal.
        let fitness = population.best().fitness();
        if fitness < best_fitness {
            best_fitness = fitness;
            stagnant_generations = 0;
        } else {
            stagnant_generations += 1;
        }

        if best_fitness <= PERFECT_FITNESS || stagnant_generations >= STAGNATION_LIMIT {
            break;
        }
    }
}

fn print_best(generation: usize, population: &Population) {
    let best = population.best();
    println!("{generation:>5}  {:.4}  {:.2?}", best.fitness(), best.genes);
}

#[cfg(test)]
mod tests;
