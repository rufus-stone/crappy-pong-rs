use rand::RngCore;

use crate::settings::*;
use genetic_algorithm as ga;
use neural_network as nn;

use ga::chromosome::*;

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

    /// Generate a neural network LayerTopology given the provided Config
    fn network_topology(config: &Config) -> [nn::topology::LayerTopology; 3] {
        [
            nn::topology::LayerTopology {
                neurons: config.data_inputs,
            },
            nn::topology::LayerTopology {
                neurons: config.brain_neurons,
            },
            nn::topology::LayerTopology {
                neurons: config.outputs,
            },
        ]
    }

    pub fn to_chromosome(&self) -> Chromosome {
        self.network.weights().collect()
    }

    /// Get immutable borrow of the brain
    pub fn network(&self) -> &nn::Network {
        &self.network
    }

    pub fn step(&self, config: &Config) -> f32 {
        1.0
    }
}
