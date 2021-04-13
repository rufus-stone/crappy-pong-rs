use rand::seq::SliceRandom;
use rand::RngCore;

use crate::individual::Individual;

#[derive(Clone, Debug, Default)]
pub struct RouletteWheelSelection;

impl RouletteWheelSelection {
    pub fn new() -> Self {
        Self
    }
}

impl SelectionMethod for RouletteWheelSelection {
    fn select<'a, I>(&self, prng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual,
    {
        population
            .choose_weighted(prng, |individual| individual.fitness())
            .expect("Population is empty!")
    }
}

pub trait SelectionMethod {
    fn select<'a, I>(&self, prng: &mut dyn RngCore, population: &'a [I]) -> &'a I
    where
        I: Individual;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::individual::TestIndividual;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    use std::collections::HashMap;

    #[test]
    fn roulette_wheel_selection() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create a RouletteWheelSelection SelectionMethod
        let roulette_wheel = RouletteWheelSelection::new();

        // Create a population of TestIndividuals with the specified fitnesses to ride the wheel
        let population = vec![
            TestIndividual::new(2.0),
            TestIndividual::new(1.0),
            TestIndividual::new(4.0),
            TestIndividual::new(3.0),
        ];

        // Spin the wheel 1000 times and see how many times each individual is chosen
        let actual = (0..1000)
            .map(|_| roulette_wheel.select(&mut prng, &population))
            .fold(
                HashMap::default(),
                |mut map: HashMap<i32, i32>, individual| {
                    *map.entry(individual.fitness() as i32).or_default() += 1;
                    map
                },
            );

        // Given the specified population and their fitness scores, after 1000 spins of the wheel we expect the TestIndividuals to have been selected the following number of times based on their fitness scores
        let expected: HashMap<i32, i32> = maplit::hashmap! {
            1 => 98, // Fitness score of 1 gets picked 98 times out of 1000 (so approx 10% of the time)
            2 => 202, // Fitness score of 2 gets picked 202 times out of 1000 (so approx 20% of the time)
            3 => 278, // Fitness score of 3 gets picked 278 times out of 1000 (so approx 30% of the time)
            4 => 422, // Fitness score of 4 gets picked 422 times out of 1000 (so approx 40% of the time)
        };

        // Check the actual selection histogram matches what we expected
        assert_eq!(actual, expected);
    }
}
