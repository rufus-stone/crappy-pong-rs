use crate::*;

#[derive(Clone, Debug)]
pub struct Statistics {
    pub(crate) min_fitness: f32,
    pub(crate) max_fitness: f32,
    pub(crate) avg_fitness: f32,
    pub(crate) sum_fitness: f32,
}

impl Statistics {
    /// Create a new Statistics struct genericised over an Individual I
    pub(crate) fn new<I>(population: &[I]) -> Self
    where
        I: Individual,
    {
        // Can't do stats on an empty population!
        assert!(!population.is_empty());

        // Use the first individual in the population to set the starting values for the stats
        let mut min_fitness = population[0].fitness();
        let mut max_fitness = min_fitness;
        let mut sum_fitness = 0.0;

        // Iterate over all the individuals in the population and compile stats about their fitness levels
        // This will end up re-checking the first individual again, but it won't affect the stats so it's not a big deal
        for individual in population {
            let fitness = individual.fitness();
            min_fitness = min_fitness.min(fitness);
            max_fitness = max_fitness.max(fitness);
            sum_fitness += fitness;
        }

        // Compute the mean average fitness
        let avg_fitness = sum_fitness / (population.len() as f32);

        // Return the calculated statistics
        Self {
            min_fitness,
            max_fitness,
            avg_fitness,
            sum_fitness,
        }
    }

    /// Return the minimum fitness of the population
    pub fn min_fitness(&self) -> f32 {
        self.min_fitness
    }

    /// Return the maximum fitness of the population
    pub fn max_fitness(&self) -> f32 {
        self.max_fitness
    }

    /// Return the mean average fitness of the population
    pub fn avg_fitness(&self) -> f32 {
        self.avg_fitness
    }

    /// Return the sum of all the fitnesses of the population
    pub fn sum_fitness(&self) -> f32 {
        self.sum_fitness
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use individual::TestIndividual;

    #[test]
    fn stats() {
        let population = vec![
            TestIndividual::new(30.0),
            TestIndividual::new(10.0),
            TestIndividual::new(20.0),
            TestIndividual::new(40.0),
        ];

        let stats = Statistics::new(&population);

        approx::assert_relative_eq!(stats.min_fitness(), 10.0);
        approx::assert_relative_eq!(stats.max_fitness(), 40.0);
        approx::assert_relative_eq!(stats.avg_fitness(), (10.0 + 20.0 + 30.0 + 40.0) / 4.0);
        approx::assert_relative_eq!(stats.sum_fitness(), 10.0 + 20.0 + 30.0 + 40.0);
    }
}
