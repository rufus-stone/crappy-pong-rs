use rand::RngCore;

use ga::statistics::Statistics;
use genetic_algorithm as ga;

use crate::players::ai::player::AiPlayer;
use crate::settings::Config;

struct Simulation {
    trainees: Vec<AiPlayer>,
}

impl Simulation {
    /// Get immutable ref to the trainees
    pub fn trainees(&self) -> &[AiPlayer] {
        &self.trainees
    }

    /// Create a new Simulation with 100 randomly generated AiPlayers
    pub fn random(config: &Config, prng: &mut dyn RngCore) -> Simulation {
        let trainees = (0..100).map(|_| AiPlayer::random(config, prng)).collect();

        Simulation { trainees }
    }

    /// Progress all the simulated games by one frame
    pub fn step(&mut self, prng: &mut dyn RngCore) -> Option<Statistics> {
        //self.process_collisions(prng);
        //self.process_brains();
        //self.process_movements();
        //self.try_evolving(prng)
        None
    }

    /// Train the AI
    pub fn train(&mut self, prng: &mut dyn RngCore) -> Statistics {
        loop {
            if let Some(statistics) = self.step(prng) {
                return statistics;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_simulation() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create a new default config
        let config = Config::default();

        // Create a new random Simulation
        let simulation = Simulation::random(&config, &mut prng);

        // Get the weights for the network of the first AiPlayer
        let weights1: Vec<f32> = simulation.trainees()[0].brain.network().weights().collect();

        // Get the weights for the network of the second AiPlayer
        let weights2: Vec<f32> = simulation.trainees()[1].brain.network().weights().collect();

        // Check that the AiPlayers are actually different
        approx::assert_relative_ne!(weights1.as_slice(), weights2.as_slice());
    }
}
