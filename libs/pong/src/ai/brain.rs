use rand::RngCore;

use genetic_algorithm as ga;
use neural_network as nn;

use ga::chromosome::*;

use super::eye::*;
use crate::settings::*;

#[derive(Clone, Debug)]
pub struct Brain {
    network: nn::Network,
}

impl Brain {
    /// Create a new brain with a random Neural Network
    pub fn random(config: &Config, rng: &mut dyn RngCore) -> Brain {
        let network = nn::Network::random(rng, &Self::network_topology(config));

        Brain { network }
    }

    /// Create a new Brain with the specified Neural Network
    pub fn new(network: nn::Network) -> Brain {
        Brain { network }
    }

    /// Create a new Brain from a Chromosome
    pub fn from_chromosome(config: &Config, chromosome: Chromosome) -> Brain {
        let network = nn::Network::from_weights(&Self::network_topology(config), chromosome);

        Brain { network }
    }

    /// Generate a neural network LayerTopology given the provided Config
    fn network_topology(config: &Config) -> [nn::topology::LayerTopology; 3] {
        [
            nn::topology::LayerTopology {
                neurons: config.eye_photoreceptors,
            },
            nn::topology::LayerTopology {
                neurons: config.brain_neurons,
            },
            nn::topology::LayerTopology {
                neurons: config.outputs,
            },
        ]
    }

    /// Generate a Chromosome from the Brain
    pub fn to_chromosome(&self) -> Chromosome {
        self.network.weights().collect()
    }

    /// Get immutable borrow of the brain
    pub fn network(&self) -> &nn::Network {
        &self.network
    }

    pub fn step(&self, config: &Config, eye: &Eye) -> f32 {
        let response = self.network.propagate(eye.photoreceptors.clone());
        response[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};
    use rand_chacha::ChaCha8Rng;

    #[test]
    fn random_brain() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // Create a new default config
        let config = Config::default();

        // Create a new random Brain
        let brain = Brain::random(&config, &mut prng);

        // A default config produces 2 layers...
        assert_eq!(brain.network().layers().len(), 2);

        // ...with 15 neurons in the first layer
        assert_eq!(brain.network().layers()[0].neurons().len(), 15);

        // ...and 1 neuron in the second layer
        assert_eq!(brain.network().layers()[1].neurons().len(), 1);

        // Check the bias of the first neuron of the first layer
        approx::assert_relative_eq!(brain.network().layers()[0].neurons()[0].bias(), -0.6255188);

        // Check the weights of the first neuron of the second layer
        let expected_weights = vec![
            -0.3431031,
            -0.8963325,
            0.053979516,
            -0.5960805,
            -0.8928735,
            -0.22718108,
            0.80183077,
            0.070950866,
            0.273749,
            -0.25688833,
            0.5900805,
            0.6363394,
            -0.29444236,
            0.5295007,
            0.4402212,
        ];
        approx::assert_relative_eq!(
            brain.network().layers()[1].neurons()[0].weights(),
            expected_weights.as_slice()
        );

        // Check the number of weights for the second neuron of the first layer (this is the same as the number of inputs it takes)
        assert_eq!(brain.network().layers()[0].neurons()[1].weights().len(), 5);
    }

    #[test]
    fn brain_from_network() {
        // Create some layers and weights
        let layers = &[
            nn::topology::LayerTopology { neurons: 3 },
            nn::topology::LayerTopology { neurons: 2 },
        ];
        let weights = vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8];

        // Create a new Network from the specified layers and weights
        let nn = nn::Network::from_weights(layers, weights.clone());

        // Create a new Brain from this Network
        let brain = Brain::new(nn);

        // Convert Brain to Chromosome
        let chromosome = brain.to_chromosome();

        // Convert the Chromosome to a Vec<f32> of weights
        let actual_weights: Vec<f32> = chromosome.into_iter().collect();

        // Check these match the weights we used to start with
        approx::assert_relative_eq!(weights.as_slice(), actual_weights.as_slice());
    }

    #[test]
    fn brain_from_chromosome() {
        // Seed a ChaCha8Rng for a predictable "random" number to use for testing
        let mut prng = ChaCha8Rng::from_seed(Default::default());

        // A default Config requires a Brain with 106 genes in its Chromosome
        let genes: Vec<f32> = (0..106).map(|_| prng.gen_range(-1.0..1.0)).collect();
        let chromosome: Chromosome = genes.into_iter().collect();

        // Create a new default config
        let config = Config::default();

        // Create a new Brain from the specified Chromosome
        let brain = Brain::from_chromosome(&config, chromosome.clone());

        // Convert Brain back to Chromosome
        let brain_chromo = brain.to_chromosome();

        // Check these match
        approx::assert_relative_eq!(brain_chromo.as_slice(), chromosome.as_slice());
    }
}
