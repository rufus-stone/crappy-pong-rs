use crossover::CrossoverMethod;
use individual::Individual;
use mutation::MutationMethod;
use rand::RngCore;
use selection::SelectionMethod;

mod chromosome;
pub mod crossover;
mod individual;
pub mod mutation;
pub mod selection;
mod statistics;

#[derive(Clone, Debug)]
pub struct GeneticAlgorithm<S, C, G> {
    selection_method: S,
    crossover_method: C,
    mutation_method: G,
}

impl<S, C, G> GeneticAlgorithm<S, C, G>
where
    S: SelectionMethod,
    C: CrossoverMethod,
    G: MutationMethod,
{
    /// Create a new GeneticAlgorithm genericised over SelectionMethod S
    pub fn new(selection_method: S, crossover_method: C, mutation_method: G) -> Self {
        Self {
            selection_method,
            crossover_method,
            mutation_method,
        }
    }

    pub fn evolve<I>(&self, prng: &mut dyn RngCore, population: &[I]) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                // Selection
                let parent_a = self.selection_method.select(prng, population).chromosome();
                let parent_b = self.selection_method.select(prng, population).chromosome();

                // Crossover
                let mut child = self.crossover_method.crossover(prng, parent_a, parent_b);

                // Mutation
                self.mutation_method.mutate(prng, &mut child);

                // Create a new individual
                I::create(child)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn individual(genes: &[f32]) -> individual::TestIndividual {
        let chromosome = genes.iter().cloned().collect();

        individual::TestIndividual::create(chromosome)
    }

    #[test]
    fn evolution() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::seed_from_u64(42);
        //let mut prng = ChaCha8Rng::from_seed(Default::default());

        let ga = GeneticAlgorithm::new(
            selection::RouletteWheelSelection::new(),
            crossover::UniformCrossover::new(),
            mutation::GaussianMutation::new(0.5, 0.5),
        );

        let mut population = vec![
            individual(&[0.0, 0.0, 0.0]),
            individual(&[1.0, 1.0, 1.0]),
            individual(&[1.0, 2.0, 1.0]),
            individual(&[1.0, 2.0, 4.0]),
        ];

        for _ in 0..10 {
            population = ga.evolve(&mut prng, &population);
        }

        /*let expected_population = vec![
            individual(&[0.44769490, 2.0648358, 4.3058133]),
            individual(&[1.21268670, 1.5538777, 2.8869110]),
            individual(&[1.06176780, 2.2657390, 4.4287640]),
            individual(&[0.95909685, 2.4618788, 4.0247330]),
        ];*/

        let expected_population = vec![
            individual(&[1.3480695, 1.941363, 3.9360921]),
            individual(&[0.34407294, 1.9081191, 3.2613263]),
            individual(&[0.34407294, 1.9828305, 3.7170746]),
            individual(&[1.3215084, 1.941363, 3.1624842]),
        ];

        assert_eq!(population, expected_population);
    }
}
